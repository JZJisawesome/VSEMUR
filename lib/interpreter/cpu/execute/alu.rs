/* alu.rs
 * By: John Jekel
 *
 * Emulates the normal (with an ALU op field) data processing instructions of the CPU (including load and store)
 * Also handles shifting (so 16 bits Shift is handled too)
 *
*/

//TODO remove this once everything is implemented
#![allow(unused_variables)]
#![allow(dead_code)]
#[allow(unused_imports)]

/* Imports */

use crate::debug_panic;
use crate::logging::log;
use crate::logging::log_noln;
use crate::logging::log_midln;
use crate::logging::log_finln;
use crate::interpreter::memory::MemoryState;
use super::super::CPUState;
use super::super::decode::*;//TODO only import what is needed from here
use super::super::decode::DecodedInstruction::*;
use DecodedALUOp::*;

/* Constants */

//TODO

/* Macros */

//TODO (also pub(crate) use the_macro statements here too)

/* Static Variables */

//TODO

/* Types */

//TODO

/* Associated Functions and Methods */

//TODO

/* Functions */

pub(super) fn execute(cpu: &mut CPUState, mem: &mut MemoryState, inst: &DecodedInstruction) {
    match inst {
        IMM16{..} | Direct16{..} | Direct6{..} | IMM6{..} | Base_plus_Disp6{..} | DS_Indirect{..} | Register{..} => { handle_big_7(cpu, mem, inst); }
        //TODO others
        _ => { debug_panic!(); }//We should not have recieved this type of instruction
    }
}

fn handle_big_7(cpu: &mut CPUState, mem: &mut MemoryState, inst: &DecodedInstruction) {
    //Operation and operands
    let operation: DecodedALUOp;
    let operand1: u16;
    let operand2: u16;
    let update_flags: bool;

    //Get the op field regardless of the instruction type
    if let IMM16{op, ..} | Direct16{op, ..} | Direct6{op, ..} | IMM6{op, ..} | Base_plus_Disp6{op, ..} | DS_Indirect{op, ..} | Register{op, ..} = inst {
        operation = *op;
    } else {
        operation = debug_panic!(DecodedALUOp::Invalid);//We should not have recieved this type of instruction (without an op field)
    }

    //Determine if we are updating flags
    if let IMM16{rd, ..} | Direct16{rd, ..} | Direct6{rd, ..} | IMM6{rd, ..} | Base_plus_Disp6{rd, ..} | DS_Indirect{rd, ..} | Register{rd, ..} = inst {
        //If it is the PC, we don't update flags, unless it is STORE, since we are writing to memory in that case. However, STORE dosn't touch flags anyways so this is fine
        update_flags = !matches!(*rd, DecodedRegister::PC);
    } else {
        update_flags = debug_panic!(false);
    }

    //Perform instruction type-specific setup, including determining operand1 and operand2
    match inst {
        IMM16{rd, rs, imm16, ..} => {
            operand1 = cpu.get_reg(*rs);
            operand2 = *imm16;
            log!(3, "Operand 1 is Rs, and operand 2 is IMM16");
        },
        Direct16{rd, w, rs, a16, ..} => {
            if *w {
                operand1 = cpu.get_reg(*rd);
                operand2 = 0;//Could be anything, it dosn't matter
                log!(3, "Operand 1 is Rd, but we don't care what operand 2 is since this is STORE");
            } else {
                operand1 = cpu.get_reg(*rs);
                operand2 = mem.read_page_addr(cpu.get_ds(), *a16);
                log!(3, "Operand 1 is Rs, and operand 2 is [A16]");
            }
        },
        Direct6{..} => {
            todo!();
        },
        IMM6{rd, imm6, ..} => {
            operand1 = cpu.get_reg(*rd);
            operand2 = *imm6 as u16;
            log!(3, "Operand 1 is Rd, and operand 2 is IMM6");
        },
        Base_plus_Disp6{rd, imm6, ..} => {
            log!(3, "Operand 1 is Rd");
            operand1 = cpu.get_reg(*rd);

            //TODO this is not needed if this is STORE (perhaps be more efficient in this case?; could also be more efficient in other cases)
            //TODO logging
            let page = cpu.get_ds();
            let bp = cpu.bp;
            let final_page_addr_tuple = super::super::inc_page_addr_by(page, bp, *imm6 as u32);

            operand2 = mem.read_page_addr(final_page_addr_tuple.0, final_page_addr_tuple.1);
        },
        DS_Indirect{rd, d, at, rs, ..} => {
            //Increment Rd if that is the @ operation we must perform
            if matches!(at, DecodedAtOp::PreIncrement) {
                let original_ds = cpu.get_ds();
                let original_rs = cpu.get_reg(*rs);
                log!(3, "@ operation says to pre-increment DS:Rs (originally {:#04X}_{:04X})", original_ds, original_rs);

                let new_ds_rs_tuple = super::super::inc_page_addr_by(original_ds, original_rs, 1);
                cpu.set_ds(new_ds_rs_tuple.0);
                cpu.set_reg(*rs, new_ds_rs_tuple.1);
            }

            log!(3, "Operand 1 is Rd");
            operand1 = cpu.get_reg(*rd);

            //Get operand2//TODO this is not needed if this is STORE (perhaps be more efficient in this case?; could also be more efficient in other cases)
            let page: u8;
            log_noln!(3, "The D flag is ");
            if *d {
                page = cpu.get_ds();
            } else {
                page = 0x00;
                log_midln!("not ");
            }
            log_finln!("set, so the page is {:#04X}", page);
            let addr: u16 = cpu.get_reg(*rs);
            log!(3, "Rs is {0:#06X}, so operand 2 is [{1:#04X}_{0:04X}]", addr, page);
            operand2 = mem.read_page_addr(page, addr);
        },
        Register{rd, sft, sfc, rs, ..} => {
            log!(3, "Operand 1 is Rd");
            operand1 = cpu.get_reg(*rd);

            let original_rs = cpu.get_reg(*rs);
            log!(3, "Rs is originally {0:#06X} | {0:#018b} | unsigned {0}", original_rs);
            log!(3, "Perform Register-type shift operations if applicable and use the result as operand 2");
            operand2 = sft_operation(*sft, *sfc, original_rs);
        }
        _ => {unimplemented!();},//TODO others
    }

    //Perform the operation
    let result: u16 = alu_operation(cpu, operation, operand1, operand2, update_flags);

    //Write to the appropriate (if any) destination
    match (operation, inst) {//TODO logging
        (CMP, _) | (TEST, _) => {},//CMP and TEST write to flags like other instructions, but the result is not stored
        (STORE, Direct16{w: true, a16, ..}) => {//Direct16 with the W flag set writes to memory
            mem.write_page_addr(result, cpu.get_ds(), *a16);
        },
        (STORE, Direct6{..}) => {
            todo!();//Store to [A6]
        },
        (STORE, Base_plus_Disp6{imm6, ..}) => {
            log!(3, "Writing result to [BP+IMM6]");
            //TODO log more info
            let page = cpu.get_ds();
            let bp = cpu.bp;
            let final_page_addr_tuple = super::super::inc_page_addr_by(page, bp, *imm6 as u32);

            mem.write_page_addr(result, final_page_addr_tuple.0, final_page_addr_tuple.1);
        },
        (STORE, DS_Indirect{d, rs, ..}) => {
            log!(3, "Writing result to {{D:}}[Rs@]");
            let page: u8;
            log_noln!(4, "The D flag is ");
            if *d {
                page = cpu.get_ds();
            } else {
                page = 0x00;
                log_midln!("not ");
            }
            log_finln!("set, so the page is {:#04X}", page);
            let addr: u16 = cpu.get_reg(*rs);
            log!(3, "Rs is {0:#06X}, so store to [{1:#04X}_{0:04X}]", addr, page);
            mem.write_page_addr(result, page, addr);
        },
        (_, IMM16{rd, ..}) | (_, Direct16{w: false, rd, ..}) | (_, Direct6{rd, ..}) | (_, IMM6{rd, ..}) | (_, Base_plus_Disp6{rd, ..}) | (_, DS_Indirect{rd, ..}) => {//Other cases are much simpler; we just write to Rd
            log!(3, "Writing result to Rd");
            cpu.set_reg(*rd, result);
        },
        (_, _) => { debug_panic!(); }//Not a valid instruction/op combination
    }

    //Potentially increment/decrement Rs if this is DS_Indirect
    if let DS_Indirect{at, rs, ..} = inst {
        if matches!(at, DecodedAtOp::PostDecrement) {
            let original_ds = cpu.get_ds();
            let original_rs = cpu.get_reg(*rs);
            log!(3, "@ operation says to post-decrement DS:Rs (originally {:#04X}_{:04X})", original_ds, original_rs);

            let new_ds_rs_tuple = super::super::dec_page_addr_by(original_ds, original_rs, 1);
            cpu.set_ds(new_ds_rs_tuple.0);
            cpu.set_reg(*rs, new_ds_rs_tuple.1);
        } else if matches!(at, DecodedAtOp::PostIncrement) {
            let original_ds = cpu.get_ds();
            let original_rs = cpu.get_reg(*rs);
            log!(3, "@ operation says to post-increment DS:Rs (originally {:#04X}_{:04X})", original_ds, original_rs);

            let new_ds_rs_tuple = super::super::inc_page_addr_by(original_ds, original_rs, 1);
            cpu.set_ds(new_ds_rs_tuple.0);
            cpu.set_reg(*rs, new_ds_rs_tuple.1);
        }
    }
}

fn alu_operation(cpu: &mut CPUState, alu_op: DecodedALUOp, operand1: u16, operand2: u16, update_flags: bool) -> u16 {//Needs mutable reference to CPUState to sets flags properly
    log!(3, "Operand 1: {0:#06X} | {0:#018b} | unsigned {0}", operand1);
    log!(3, "Operand 2: {0:#06X} | {0:#018b} | unsigned {0}", operand2);

    use std::num::Wrapping as Wrap;

    //We need regular wrapping behaviour to make our lives easier; also do 32 bit operations so we get the carry bit (which is useful) for free
    let operand1_w = Wrap(operand1 as u32);
    let operand2_w = Wrap(operand2 as u32);

    //Perform operation
    let result_w: Wrap<u32>;
    match alu_op {
        ADD => { result_w = operand1_w + operand2_w; },
        ADC => { result_w = operand1_w + operand2_w + if cpu.get_c() { Wrap(1) } else { Wrap(0) }; },
        SUB => { result_w = operand1_w - operand2_w; },
        SBC => { result_w = operand1_w + !operand2_w + if cpu.get_c() { Wrap(1) } else { Wrap(0) }; },
        CMP => { result_w = operand1_w - operand2_w; },
        NEG => { result_w = Wrap((-(operand2 as i32)) as u32); },//Intentionally not using operand2_w so that we can cast to a signed integer and back//TODO ensure this is valid, else do ~operand2 + 1
        XOR => { result_w = operand1_w ^ operand2_w; },
        LOAD => { result_w = operand2_w; },
        OR => { result_w = operand1_w | operand2_w; },
        AND => { result_w = operand1_w & operand2_w; },
        TEST => { result_w = operand1_w & operand2_w; },
        STORE => { result_w = operand1_w; },
        _ => { result_w = debug_panic!(Wrap(0)); },
    }
    let result: u32 = result_w.0;//We don't need wrapping behaviour anymore
    log!(3, "Result: {0:#06X} | {0:#018b} | unsigned {0}", (result & 0xFFFF) as u16);

    //Set flags
    //TODO logging for flag updates
    //N flag is set if the result's msb is 1
    //Z flag is set if the result is 0
    //S flag is set if the result is negative (not the same as N since it looks at higher bits too)
    //C flag is set if there was a carry
    if update_flags {
        log!(3, "Updating flags...");
        match alu_op {
            ADD | ADC | SUB | SBC | CMP => {//ADD, ADC, SUB, SBC, CMP update all flags
                cpu.set_n(((result >> 15) & 0b1) == 0b1);
                cpu.set_z(result == 0);
                cpu.set_s((result as i32) < 0);//TODO ensure this is correct; mame does this differently
                cpu.set_c(((result >> 16) & 0b1) == 0b1);
            },
            NEG | XOR | LOAD | OR | AND | TEST => {//NEG, XOR, LOAD, OR, AND, TEST update only N, Z flags
                cpu.set_n(((result >> 15) & 0b1) == 0b1);
                cpu.set_z(result == 0);
            },
            STORE => {},//STORE dosn't update flags
            _ => { return 0; },//TODO should we do some sort of error handling for this, or do we need to jump somewhere if this occurs?
        }
    }


    return (result & 0xFFFF) as u16;
}

fn sft_operation(sft: DecodedSFTOp, sfc: u8, rs: u16) -> u16 {
    use DecodedSFTOp::*;
    let shift_amount = sfc + 1;
    match sft {
        NOP => { return rs; },
        ASR => { return ((rs as i16) >> shift_amount) as u16; },
        LSL => { return rs << shift_amount; },
        LSR => { return rs >> shift_amount; },
        ROL => { return rs.rotate_left(shift_amount as u32); },
        ROR => { return rs.rotate_right(shift_amount as u32); },

        Invalid => { return debug_panic!(0); },
    }
    todo!();
}
