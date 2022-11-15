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
        _ => { debug_panic!(); }//We should not have recieved this type of instruction
    }
}

fn handle_big_5(cpu: &mut CPUState, mem: &mut MemoryState, inst: &DecodedInstruction) {
    //Operation and operands
    let operation: DecodedALUOp;
    let operand1: u16;
    let operand2: u16;

    //Get the op field regardless of the instruction type
    if let IMM16{op, ..} | Direct16{op, ..} | IMM6{op, ..} = inst {//TODO others
        operation = *op;
    } else {
        operation = debug_panic!(DecodedALUOp::Invalid);//We should not have recieved this type of instruction (without an op field)
    }

    //Perform instruction type-specific setup
    match inst {
        IMM16{rs, imm16, ..} => {
            operand1 = cpu.get_reg(*rs);
            operand2 = *imm16;
            log!(3, "Operand 1 is Rs, and operand 2 is IMM16");
        },
        Direct16{rd, w, rs, a16, ..} => {
            if *w {
                debug_assert!(matches!(operation, STORE));//TODO confirm this is a valid asumption
                operand1 = cpu.get_reg(*rd);
                operand2 = cpu.get_reg(*rs);
                log!(4, "Operand 1 is Rd, and operand 2 is Rs");
            } else {
                operand1 = cpu.get_reg(*rs);
                operand2 = mem.read_page_addr(cpu.get_ds(), *a16);
                log!(4, "Operand 1 is Rs, and operand 2 is [A16]");
            }
        },
        IMM6{rd, imm6, ..} => {
            operand1 = cpu.get_reg(*rd);
            operand2 = *imm6 as u16;
            log!(3, "Operand 1 is Rd, and operand 2 is IMM6");
        },
        _ => {unimplemented!();},//TODO
    }

    //Perform the operation
    let result: u16 = alu_operation(cpu, operation, operand1, operand2);

    //Write to the appropriate (if any) destination
    match (operation, inst) {
        (CMP, _) | (TEST, _) => {},//CMP and TEST write to flags like other instructions, but the result is not stored

        //TODO others//NOTE: STOREs can occur with Direct16 (w flag must be set), Direct6, Base+Disp6, and DS_Indirect only

        (_, Direct16{w: true, a16, ..}) => {//Direct16 with the W flag set writes to memory
            debug_assert!(matches!(operation, STORE));//TODO confirm this is a valid asumption
            mem.write_page_addr(result, cpu.get_ds(), *a16);
        },
        (_, IMM16{rd, ..}) | (_, Direct16{w: false, rd, ..}) | (_, Direct6{rd, ..}) | (_, IMM6{rd, ..}) | (_, Base_plus_Disp6{rd, ..}) => {//Other cases are much simpler; we just write to Rd
            cpu.set_reg(*rd, result);
        }
        (_, _) => { debug_panic!(); }//Not a valid instruction/op combination
    }
}

fn alu_operation(cpu: &mut CPUState, alu_op: DecodedALUOp, operand1: u16, operand2: u16) -> u16 {//Needs mutable reference to CPUState to sets flags properly
    log!(4, "Operand 1: {0:#06X} | {0:#018b} | unsigned {0}", operand1);
    log!(4, "Operand 2: {0:#06X} | {0:#018b} | unsigned {0}", operand2);

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
