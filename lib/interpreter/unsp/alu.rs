/* alu.rs
 * Copyright (C) 2022 John Jekel
 * See the LICENSE file at the root of the project for licensing info.
 *
 * Emulates the normal (with an ALU op field) data processing instructions of the CPU (including load and store)
 * Also handles shifting (so 16 bits Shift is handled too)
 *
*/

//TODO remove this once everything is implemented
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(dead_code)]

/* Imports */

use crate::debug_panic;
use crate::logging::log;
use crate::logging::log_noln;
use crate::logging::log_midln;
use crate::logging::log_finln;
use crate::interpreter::common::CPU;
use crate::interpreter::common::ReadableMemory;
use crate::interpreter::common::WritableMemory;
use crate::interpreter::common::inc_page_addr_by;
use crate::interpreter::common::dec_page_addr_by;
use crate::decode::*;//TODO only import what is needed from here
use crate::decode::DecodedInstruction::*;
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

pub(super) fn execute(state: &mut (impl CPU + ReadableMemory + WritableMemory), inst: &DecodedInstruction) -> u8 {
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

    //Perform instruction type-specific setup, including determining operand1 and operand2 and incrementing the PC
    match inst {
        IMM16{rd, rs, imm16, ..} => {
            operand1 = state.get_reg(*rs);
            operand2 = *imm16;
            log!(3, "Operand 1 is Rs, and operand 2 is IMM16");
            state.inc_pc_by(2);
        },
        Direct16{rd, w, rs, a16, ..} => {
            if *w {
                operand1 = state.get_reg(*rd);
                operand2 = state.get_reg(*rs);
                log!(3, "Operand 1 is Rd, and operand 2 is Rs");
            } else {
                operand1 = state.get_reg(*rs);
                operand2 = state.read_page_addr(0x00, *a16);
                log!(3, "Operand 1 is Rs, and operand 2 is [A16]");
            }
            state.inc_pc_by(2);
        },
        Direct6{..} => {
            todo!();
            //state.inc_pc();
        },
        IMM6{rd, imm6, ..} => {
            operand1 = state.get_reg(*rd);
            operand2 = *imm6 as u16;
            log!(3, "Operand 1 is Rd, and operand 2 is IMM6");
            state.inc_pc();
        },
        Base_plus_Disp6{rd, imm6, ..} => {
            log!(3, "Operand 1 is Rd");
            operand1 = state.get_reg(*rd);

            //TODO this is not needed if this is STORE (perhaps be more efficient in this case?; could also be more efficient in other cases)
            //TODO logging
            let page = 0x00;
            let bp = state.get_bp();
            let final_page_addr_tuple = inc_page_addr_by(page, bp, *imm6 as u32);

            operand2 = state.read_page_addr(final_page_addr_tuple.0, final_page_addr_tuple.1);
            state.inc_pc();
        },
        DS_Indirect{rd, d, at, rs, ..} => {
            //Increment Rd if that is the @ operation we must perform
            if matches!(at, DecodedAtOp::PreIncrement) {
                let original_ds = state.get_ds();
                let original_rs = state.get_reg(*rs);
                log!(3, "@ operation says to pre-increment DS:Rs (originally {:#04X}_{:04X})", original_ds, original_rs);

                let new_ds_rs_tuple = inc_page_addr_by(original_ds, original_rs, 1);
                state.set_ds(new_ds_rs_tuple.0);
                state.set_reg(*rs, new_ds_rs_tuple.1);
            }

            log!(3, "Operand 1 is Rd");
            operand1 = state.get_reg(*rd);

            //Get operand2//TODO this is not needed if this is STORE (perhaps be more efficient in this case?; could also be more efficient in other cases)
            let page: u8;
            log_noln!(3, "The D flag is ");
            if *d {
                page = state.get_ds();
            } else {
                page = 0x00;
                log_midln!("not ");
            }
            log_finln!("set, so the page is {:#04X}", page);
            let addr: u16 = state.get_reg(*rs);
            log!(3, "Rs is {0:#06X}, so operand 2 is [{1:#04X}_{0:04X}]", addr, page);
            operand2 = state.read_page_addr(page, addr);
            state.inc_pc();
        },
        Register{rd, sft, sfc, rs, ..} => {
            log!(3, "Operand 1 is Rd");
            operand1 = state.get_reg(*rd);

            let original_rs = state.get_reg(*rs);
            log!(3, "Rs is originally {0:#06X} | {0:#018b} | unsigned {0}", original_rs);
            log!(3, "Perform Register-type shift operations if applicable and use the result as operand 2");
            operand2 = sft_operation(*sft, *sfc, original_rs);
            state.inc_pc();
        }
        _ => {unimplemented!();},//TODO others
    }

    //Perform the operation
    let result: u16 = alu_operation(state, operation, operand1, operand2, update_flags);

    //Write to the appropriate (if any) destination,
    let write_to_pc: bool;
    match (operation, inst) {//TODO logging
        (CMP, _) | (TEST, _) => { write_to_pc = false; },//CMP and TEST write to flags like other instructions, but the result is not stored
        (STORE, Direct6{..}) => {
            todo!();//Store to [A6]
            //write_to_pc = false;
        },
        (STORE, Base_plus_Disp6{imm6, ..}) => {
            //TODO logging
            log!(3, "Writing result to [BP+IMM6]");
            //TODO log more info
            let page = 0x00;
            let bp = state.get_bp();
            let final_page_addr_tuple = inc_page_addr_by(page, bp, *imm6 as u32);

            state.write_page_addr(final_page_addr_tuple.0, final_page_addr_tuple.1, result);
            write_to_pc = false;
        },
        (STORE, DS_Indirect{d, rs, ..}) => {
            log!(3, "Writing result to {{D:}}[Rs@]");
            let page: u8;
            log_noln!(4, "The D flag is ");
            if *d {
                page = state.get_ds();
            } else {
                page = 0x00;
                log_midln!("not ");
            }
            log_finln!("set, so the page is {:#04X}", page);
            let addr: u16 = state.get_reg(*rs);
            log!(3, "Rs is {0:#06X}, so store to [{1:#04X}_{0:04X}]", addr, page);
            state.write_page_addr(page, addr, result);
            write_to_pc = false;
        },
        (LOAD, Direct16{w: true, a16, ..}) | (STORE, Direct16{w: false, a16, ..}) => { write_to_pc = debug_panic!(false); }//Not a valid instruction/op combination
        (_, Direct16{w: true, a16, ..}) => {//When the Direct16 w flag is set, we are writing to memory
            //TODO logging
            state.write_page_addr(0x00, *a16, result);
            write_to_pc = false;
        },
        (_, IMM16{rd, ..}) | (_, Direct16{w: false, rd, ..}) | (_, Direct6{rd, ..}) | (_, IMM6{rd, ..}) | (_, Base_plus_Disp6{rd, ..}) | (_, DS_Indirect{rd, ..}) | (_, Register{rd, ..}) => {
            //Other cases are much simpler; we just write to Rd
            log!(3, "Writing result to Rd");
            state.set_reg(*rd, result);
            write_to_pc = matches!(*rd, DecodedRegister::PC);
        },
        (_, _) => { write_to_pc = debug_panic!(false); }//Not a valid instruction/op combination
    }

    //Potentially increment/decrement Rs if this is DS_Indirect
    if let DS_Indirect{at, rs, ..} = inst {
        if matches!(at, DecodedAtOp::PostDecrement) {
            let original_ds = state.get_ds();
            let original_rs = state.get_reg(*rs);
            log!(3, "@ operation says to post-decrement DS:Rs (originally {:#04X}_{:04X})", original_ds, original_rs);

            let new_ds_rs_tuple = dec_page_addr_by(original_ds, original_rs, 1);
            state.set_ds(new_ds_rs_tuple.0);
            state.set_reg(*rs, new_ds_rs_tuple.1);
        } else if matches!(at, DecodedAtOp::PostIncrement) {
            let original_ds = state.get_ds();
            let original_rs = state.get_reg(*rs);
            log!(3, "@ operation says to post-increment DS:Rs (originally {:#04X}_{:04X})", original_ds, original_rs);

            let new_ds_rs_tuple = inc_page_addr_by(original_ds, original_rs, 1);
            state.set_ds(new_ds_rs_tuple.0);
            state.set_reg(*rs, new_ds_rs_tuple.1);
        }
    }

    return get_cycle_count(inst, write_to_pc);
}

fn alu_operation(state: &mut impl CPU, alu_op: DecodedALUOp, operand1: u16, operand2: u16, update_flags: bool) -> u16 {//Needs mutable reference to CPUState to sets flags properly
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
        ADC => { result_w = operand1_w + operand2_w + if state.get_c() { Wrap(1) } else { Wrap(0) }; },
        SUB => { result_w = operand1_w - operand2_w; },
        SBC => { result_w = operand1_w + !operand2_w + if state.get_c() { Wrap(1) } else { Wrap(0) }; },
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
                state.set_n(((result >> 15) & 0b1) == 0b1);
                state.set_z(result == 0);
                state.set_s((result as i32) < 0);//TODO ensure this is correct; mame does this differently
                state.set_c(((result >> 16) & 0b1) == 0b1);
            },
            NEG | XOR | LOAD | OR | AND | TEST => {//NEG, XOR, LOAD, OR, AND, TEST update only N, Z flags
                state.set_n(((result >> 15) & 0b1) == 0b1);
                state.set_z(result == 0);
            },
            STORE => {},//STORE dosn't update flags
            _ => { return debug_panic!(0); },//TODO should we do some sort of error handling for this, or do we need to jump somewhere if this occurs?
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
}

fn get_cycle_count(inst: &DecodedInstruction, write_to_pc: bool) -> u8 {
    match (inst, write_to_pc) {
        (IMM16{..}, false) => { return 4; },
        (Direct16{..}, false) => { return 7; },
        (Direct6{..}, false) => { return 5; },
        (DS_Indirect{..}, false) => { return 6; },
        (Register{..}, false) => { return 3; },
        (IMM16{..}, true) => { return 5; },
        (Direct16{..}, true) => { return 8; },
        (Direct6{..}, true) => { return 6; },
        (DS_Indirect{..}, true) => { return 7; },
        (Register{..}, true) => { return 5; },

        (IMM6{..}, _) => { return 2; },
        (Base_plus_Disp6{..}, _) => { return 6; },

        _ => { return debug_panic!(0); },
    }
}
