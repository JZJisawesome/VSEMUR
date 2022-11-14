/* execute.rs
 * By: John Jekel
 *
 * Module for executing unSP instructions and performing other operations
 *
*/

//TODO remove this once everything is implemented
#![allow(unused_variables)]
#![allow(dead_code)]

/* Imports */

use crate::logging::log;
use crate::logging::log_noln;
use crate::logging::log_finln;
use crate::interpreter::memory::MemoryState;
use super::CPUState;
use super::decode::*;//TODO only import what is needed
use super::decode::DecodedInstruction::*;

/* Constants */

//TODO

/* Macros */

/* Static Variables */

//TODO

/* Types */

//TODO

/* Associated Functions and Methods */

//TODO

/* Functions */

pub(super) fn execute(cpu: &mut CPUState, mem: &mut MemoryState, inst: &DecodedInstruction) {
    log!(1, "CPU: Execute instruction");

    perform_instruction(cpu, mem, inst);
    cpu.set_cycle_count(get_cycle_count(inst));
    increment_pc(cpu, inst);
}

fn perform_instruction(cpu: &mut CPUState, mem: &mut MemoryState, inst: &DecodedInstruction) {
    log!(2, "Perform instruction operations");
    match inst {
        DSI6{..} => {
            unimplemented!();//TODO
        },
        CALL{..} => {
            unimplemented!();//TODO
        },
        JMPF{..} => {
            unimplemented!();//TODO
        },
        JMPR{..} => {
            unimplemented!();//TODO
        },
        FIR_MOV{..}=> {
            unimplemented!();//TODO
        },
        Fraction{..} => {
            unimplemented!();//TODO
        },
        INT_SET{..} => {
            unimplemented!();//TODO
        },
        IRQ{..} => {
            unimplemented!();//TODO
        },
        SECBANK{..} => {
            unimplemented!();//TODO
        },
        FIQ{..} => {
            unimplemented!();//TODO
        },
        IRQ_Nest_Mode{..} => {
            unimplemented!();//TODO
        },
        BREAK{..} => {
            unimplemented!();//TODO
        },
        CALLR{..} => {
            unimplemented!();//TODO
        },
        DIVS{..} => {
            unimplemented!();//TODO
        },
        DIVQ{..} => {
            unimplemented!();//TODO
        },
        EXP{..} => {
            unimplemented!();//TODO
        },
        NOP => { /* We don't need to do anything! :) */ },
        DS_Access{..} => {
            unimplemented!();//TODO
        },
        FR_Access{..} => {
            unimplemented!();//TODO
        },
        MUL{..} => {
            unimplemented!();//TODO
        },
        MULS{..} => {
            unimplemented!();//TODO
        },
        Register_BITOP_Rs{..} => {
            unimplemented!();//TODO
        },
        Register_BITOP_offset{..} => {
            unimplemented!();//TODO
        },
        Memory_BITOP_offset{..} => {
            unimplemented!();//TODO
        },
        Memory_BITOP_Rs{..} => {
            unimplemented!();//TODO
        },
        sixteen_bits_Shift{..} => {
            unimplemented!();//TODO
        },
        RETI{..} => {
            unimplemented!();//TODO
        },
        RETF{..} => {
            unimplemented!();//TODO
        },
        Base_plus_Disp6{..} => {
            unimplemented!();//TODO
        },
        IMM6{..} => {
            unimplemented!();//TODO
        },
        Branch{..} => {
            unimplemented!();//TODO
        },
        Stack_Operation{..} => {
            unimplemented!();//TODO
        },
        DS_Indirect{..} => {
            unimplemented!();//TODO
        },
        IMM16{..} => {
            unimplemented!();//TODO
        },
        Direct16{..} => {
            unimplemented!();//TODO
        },
        Direct6{..} => {
            unimplemented!();//TODO
        },
        Register{..} => {
            unimplemented!();//TODO
        },

        InvalidInstructionType => { panic!(); }//TODO proper error handling
    }
}

fn get_cycle_count(inst: &DecodedInstruction) -> u8 {
    log!(2, "Determine clock cycles it will take to execute");
    match inst {
        //TODO combine into individual cases more efficiently (to make more readable)
        DSI6{..} => { return 2; },
        CALL{..} => { return 9; },
        JMPF{..} => { return 5; },
        JMPR{..} => { return 4; },
        FIR_MOV{..}=> { return 2; },
        Fraction{..} => { return 2; },
        INT_SET{..} => { return 2; },
        IRQ{..} => { return 2; },
        SECBANK{..} => { return 2; },
        FIQ{..} => { return 2; },
        IRQ_Nest_Mode{..} => { return 2; },
        BREAK{..} => { return 10; },
        CALLR{..} => { return 8; },
        DIVS{..} => { return 2; }
        DIVQ{..} => { return 3; }
        EXP{..} => { return 2; }
        NOP{..} => { return 2; },
        DS_Access{..} => { return 2; },
        FR_Access{..} => { return 2; },
        MUL{..} => { return 12; },
        MULS{..} => {
            unimplemented!();//TODO
        },
        Register_BITOP_Rs{..} => { return 4; }
        Register_BITOP_offset{..} => { return 4; }
        Memory_BITOP_offset{..} => { return 7; }
        Memory_BITOP_Rs{..} => { return 7; }
        sixteen_bits_Shift{..} => { return 8; },
        RETI{..} => {
            unimplemented!();//TODO
        },
        RETF{..} => { return 8; },
        Base_plus_Disp6{..} => { return 6; },
        IMM6{..} => { return 2; },
        Branch{..} => {
            unimplemented!();//TODO
        },
        Stack_Operation{size, ..} => { return (2 * size) + 4; },
        DS_Indirect{rd, ..} => {
            if matches!(rd, DecodedRegister::PC) {
                return 7;
            } else {
                return 6;
            }
        },
        IMM16{rd, ..} => {
            if matches!(rd, DecodedRegister::PC) {
                return 5;
            } else {
                return 4;
            }
        },
        Direct16{rd, ..} => {
            if matches!(rd, DecodedRegister::PC) {
                return 8;
            } else {
                return 7;
            }
        },
        Direct6{rd, ..} => {
            if matches!(rd, DecodedRegister::PC) {
                return 6;
            } else {
                return 5;
            }
        },
        Register{rd, ..} => {
            if matches!(rd, DecodedRegister::PC) {
                return 5;
            } else {
                return 3;
            }
        },

        InvalidInstructionType => { panic!(); }//TODO proper error handling
    }
}

fn increment_pc(cpu: &mut CPUState, inst: &DecodedInstruction) {
    log!(2, "Increment PC if applicable");
    match inst {
        DSI6{..} => {
            unimplemented!();//TODO
        },
        CALL{..} => {
            unimplemented!();//TODO
        },
        JMPF{..} => {
            unimplemented!();//TODO
        },
        JMPR{..} => {
            unimplemented!();//TODO
        },
        FIR_MOV{..}=> {
            unimplemented!();//TODO
        },
        Fraction{..} => {
            unimplemented!();//TODO
        },
        INT_SET{..} => {
            unimplemented!();//TODO
        },
        IRQ{..} => {
            unimplemented!();//TODO
        },
        SECBANK{..} => {
            unimplemented!();//TODO
        },
        FIQ{..} => {
            unimplemented!();//TODO
        },
        IRQ_Nest_Mode{..} => {
            unimplemented!();//TODO
        },
        BREAK{..} => {
            unimplemented!();//TODO
        },
        CALLR{..} => {
            unimplemented!();//TODO
        },
        DIVS{..} => {
            unimplemented!();//TODO
        },
        DIVQ{..} => {
            unimplemented!();//TODO
        },
        EXP{..} => {
            unimplemented!();//TODO
        },
        NOP{..} => {
            unimplemented!();//TODO
        },
        DS_Access{..} => {
            unimplemented!();//TODO
        },
        FR_Access{..} => {
            unimplemented!();//TODO
        },
        MUL{..} => {
            unimplemented!();//TODO
        },
        MULS{..} => {
            unimplemented!();//TODO
        },
        Register_BITOP_Rs{..} => {
            unimplemented!();//TODO
        },
        Register_BITOP_offset{..} => {
            unimplemented!();//TODO
        },
        Memory_BITOP_offset{..} => {
            unimplemented!();//TODO
        },
        Memory_BITOP_Rs{..} => {
            unimplemented!();//TODO
        },
        sixteen_bits_Shift{..} => {
            unimplemented!();//TODO
        },
        RETI{..} => {
            unimplemented!();//TODO
        },
        RETF{..} => {
            unimplemented!();//TODO
        },
        Base_plus_Disp6{..} => {
            unimplemented!();//TODO
        },
        IMM6{..} => {
            unimplemented!();//TODO
        },
        Branch{..} => {
            unimplemented!();//TODO
        },
        Stack_Operation{..} => {
            unimplemented!();//TODO
        },
        DS_Indirect{..} => {
            unimplemented!();//TODO
        },
        IMM16{..} => {
            unimplemented!();//TODO
        },
        Direct16{..} => {
            unimplemented!();//TODO
        },
        Direct6{..} => {
            unimplemented!();//TODO
        },
        Register{..} => {
            unimplemented!();//TODO
        },

        InvalidInstructionType => { panic!(); }//TODO proper error handling
    }
}
