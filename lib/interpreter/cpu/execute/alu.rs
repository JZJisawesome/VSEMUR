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

use crate::logging::log;
use crate::logging::log_noln;
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
        IMM16{..} | Direct16{..} | Direct6{..} | IMM6{..} | Base_plus_Disp6{..} => { handle_big_5(cpu, mem, inst); }
        //TODO others
        _ => { panic!(); }//We should not have recieved this type of instruction
    }
}

fn handle_big_5(cpu: &mut CPUState, mem: &mut MemoryState, inst: &DecodedInstruction) {
    //Operation and operands
    let operation: DecodedALUOp;
    let operand1: u16;
    let operand2: u16;

    //Get the op field regardless of the instruction type, and also perform instruction type-specific setup
    match inst {
        IMM16{op, rd, rs, imm16} => {
            operation = *op;
            operand1 = cpu.get_reg(*rs);
            operand2 = *imm16;
            //TODO logging
        },
        _ => {unimplemented!();},//TODO
    }

    //Perform the operation
    let result: u16 = alu_operation(cpu, operation, operand1, operand2);

    //Write to the appropriate (if any) destination
    match (operation, inst) {
        (CMP, _) | (TEST, _) => {},//CMP and TEST write to flags like other instructions, but not to Rd/to memory
        //TODO others
        (_, IMM16{rd, ..}) | (_, Direct16{rd, ..}) | (_, Direct6{rd, ..}) | (_, IMM6{rd, ..}) | (_, Base_plus_Disp6{rd, ..}) => {//Other cases are much simpler; we just write to Rd
            cpu.set_reg(*rd, result);
        }
        (_, _) => { panic!(); }
    }

    /*
    //Write to the appropriate (if any) destination
    match (upper_nibble, direct16, direct16_w) {
        (0b0100, _, _) | (0b1100, _, _) => {},//CMP and TEST write to flags like other instructions, but not to Rd/to memory
        (0b1101, false, _) => {//IMM16 STORE is invalid (we can't store to an immediate)//TODO should we do some sort of error handling for this (TickFail?), or do we need to jump somewhere if this occurs?
            log!(5, "This isn't valid: we can't store a result to an immediate!");
            panic!();
        },
        (0b1101, true, true) => {//Direct16 STORE + w flag set stores the result (which is Rd) to Rs
            let rs_index: u8 = rs_index!(inst_word);
            set_reg_by_index(cpu, rs_index, result);//rs is rd, and rd is result
            log_register!(5, "Rs", rs_index, result);
            //TODO cycle count in this case
        },
        (0b1101, true, false) |//Direct16 STORE + w flag not set stores the result (which is Rs) to memory
        (_, true, true) => {//Direct16 operation with w flag set writes result to memory instead of a register
            unimplemented!();//TODO (also cycle count in this case)
        }
        (_, false, _) | (_, true, false) => {//Other cases are much simpler; we just write to the destination register
            let rd_index: u8 = rd_index!(inst_word);
            set_reg_by_index(cpu, rd_index, result);
            log_register!(5, "Rd", rd_index, result);

            //Determine cycle count in this case
            if direct16 {
                cpu.set_cycle_count(if rd_index == 0b111 { 5 } else { 4 });
            } else {
                cpu.set_cycle_count(if rd_index == 0b111 { 8 } else { 7 });
            }
        }
    }
    */
}

fn alu_operation(cpu: &mut CPUState, alu_op: DecodedALUOp, operand1: u16, operand2: u16) -> u16 {//Needs mutable reference to CPUState to sets flags properly
    use std::num::Wrapping as Wrap;

    //We need regular wrapping behaviour to make our lives easier; also do 32 bit operations so we get the carry bit (which is useful) for free
    let operand1_w = Wrap(operand1 as u32);
    let operand2_w = Wrap(operand2 as u32);

    //Perform operation
    log_noln!(4, "Operation: ");
    let result_w: Wrap<u32>;
    match alu_op {
        ADD => {
            log_finln!("ADD");
            result_w = operand1_w + operand2_w;
        },
        ADC => {
            log_finln!("ADC");
            result_w = operand1_w + operand2_w + if cpu.get_c() { Wrap(1) } else { Wrap(0) };
        },
        SUB => {
            log_finln!("SUB");
            result_w = operand1_w - operand2_w;
        },
        SBC => {
            log_finln!("SBC");
            result_w = operand1_w + !operand2_w + if cpu.get_c() { Wrap(1) } else { Wrap(0) };
        },
        CMP => {
            log_finln!("CMP");
            result_w = operand1_w - operand2_w;
        },
        NEG => {
            log_finln!("NEG");
            result_w = Wrap((-(operand2 as i32)) as u32);//Intentionally not using operand2_w so that we can cast to a signed integer and back//TODO ensure this is valid, else do ~operand2 + 1
        },
        XOR => {
            log_finln!("XOR");
            result_w = operand1_w ^ operand2_w;
        },
        LOAD => {
            log_finln!("LOAD");
            result_w = operand2_w;
        },
        OR => {
            log_finln!("OR");
            result_w = operand1_w | operand2_w;
        },
        AND => {
            log_finln!("AND");
            result_w = operand1_w & operand2_w;
        },
        TEST => {
            log_finln!("TEST");
            result_w = operand1_w & operand2_w;
        },
        STORE => {
            log_finln!("STORE");
            result_w = operand1_w;//No need for any flags to be set with store
        },
        _ => { panic!(); },
    }
    let result: u32 = result_w.0;//We don't need wrapping behaviour anymore
    log!(4, "Result:{:#06X} | {:#018b} | unsigned {}", (result & 0xFFFF) as u16, (result & 0xFFFF) as u16, (result & 0xFFFF) as u16);

    //Set flags
    //FIXME don't update flags if the register is the PC
    //N flag is set if the result's msb is 1
    //Z flag is set if the result is 0
    //S flag is set if the result is negative (not the same as N since it looks at higher bits too)
    //C flag is set if there was a carry
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


    return (result & 0xFFFF) as u16;
}
