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

mod alu;
mod bitop;
mod control;
mod muldiv;
mod stack;

use crate::debug_panic;
use crate::logging::log;
use crate::logging::log_noln;
use crate::logging::log_finln;
use crate::interpreter::memory::MemoryState;
use super::CPUState;
use super::decode::*;//TODO only import what is needed from here
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
        sixteen_bits_Shift{..} | Base_plus_Disp6{..} | IMM6{..} | DS_Indirect{..} | IMM16{..} | Direct16{..} | Direct6{..} | Register{..} => {
            alu::execute(cpu, mem, inst);
        },
        Register_BITOP_Rs{..} | Register_BITOP_offset{..} | Memory_BITOP_offset{..} | Memory_BITOP_Rs{..} => {
            bitop::execute(cpu, mem, inst);
        },
        CALL{..} | JMPF{..} | JMPR{..} | BREAK{..} | CALLR{..} | RETI{..} | RETF{..} | Branch{..} => {
            control::execute(cpu, mem, inst);
        },
        DIVS{..} | DIVQ{..} | EXP{..} | MUL{..} | MULS{..} => {
            muldiv::execute(cpu, mem, inst);
        },
        Stack_Operation{..} => {
            stack::execute(cpu, mem, inst);
        },
        DSI6{imm6} => { cpu.set_ds(*imm6); },
        FIR_MOV{fir}=> { cpu.set_fir(*fir); },
        Fraction{fra} => { cpu.set_fra(*fra); },
        INT_SET{f, i} => {
            cpu.set_fiq(*f);
            cpu.set_irq(*i);
        },
        IRQ{i} => { cpu.set_irq(*i); },
        SECBANK{s} => { cpu.set_bnk(*s); },
        FIQ{f} => { cpu.set_fiq(*f); },
        IRQ_Nest_Mode{n} => { cpu.set_ine(*n); },
        NOP => { /* We don't need to do anything! :) */ },
        DS_Access{w, rs} => {
            unimplemented!();//TODO do here
        },
        FR_Access{w, rs} => {
            unimplemented!();//TODO do here
        },

        Invalid => { debug_panic!(); }//TODO proper error handling?
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
            unimplemented!();//TODO figure out what N means
        },
        Register_BITOP_Rs{..} => { return 4; }
        Register_BITOP_offset{..} => { return 4; }
        Memory_BITOP_offset{..} => { return 7; }
        Memory_BITOP_Rs{..} => { return 7; }
        sixteen_bits_Shift{..} => { return 8; },
        RETI{..} => {
            unimplemented!();//TODO requires accessing CPU state
        },
        RETF{..} => { return 8; },
        Base_plus_Disp6{..} => { return 6; },
        IMM6{..} => { return 2; },
        Branch{..} => {
            unimplemented!();//TODO depends on if branch is taken or not
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

        Invalid => { return debug_panic!(0); }//TODO proper error handling?
    }
}

fn increment_pc(cpu: &mut CPUState, inst: &DecodedInstruction) {
    log!(2, "Increment PC if applicable");
    match inst {
        //TODO combine into individual cases more efficiently (to make more readable)
        CALL{..} | JMPF{..} | JMPR{..} | BREAK{..} | CALLR{..} | RETI{..} | RETF{..} | Branch{..} => {/* PC Not Modified Here */},

        DSI6{..} | FIR_MOV{..} | Fraction{..} | INT_SET{..} | IRQ{..} | SECBANK{..} | FIQ{..} |
        IRQ_Nest_Mode{..} | DIVS{..} | DIVQ{..} | EXP{..} | NOP{..} => {
            cpu.inc_pc();
        },

        DS_Access{..} => {
            unimplemented!();//TODO what if Rs is the PC?
        },
        FR_Access{..} => {
            unimplemented!();//TODO what if Rs is the PC?
        },

        MUL{rd, ..} => {
            if matches!(rd, DecodedRegister::PC) {
                unimplemented!();//TODO what is the behaviour in this case?
            } else {
                cpu.inc_pc();
            }
        },
        MULS{rd, ..} => {
            if matches!(rd, DecodedRegister::PC) {
                unimplemented!();//TODO what is the behaviour in this case?
            } else {
                cpu.inc_pc();
            }
        },
        Register_BITOP_Rs{rd, ..} => {
            if matches!(rd, DecodedRegister::PC) {
                unimplemented!();//TODO what is the behaviour in this case?
            } else {
                cpu.inc_pc();
            }
        },
        Register_BITOP_offset{rd, ..} => {
            if matches!(rd, DecodedRegister::PC) {
                unimplemented!();//TODO what is the behaviour in this case?
            } else {
                cpu.inc_pc();
            }
        },
        Memory_BITOP_offset{rd, ..} => {
            if matches!(rd, DecodedRegister::PC) {
                unimplemented!();//TODO what is the behaviour in this case?
            } else {
                cpu.inc_pc();
            }
        },
        Memory_BITOP_Rs{rd, ..} => {
            if matches!(rd, DecodedRegister::PC) {
                unimplemented!();//TODO what is the behaviour in this case?
            } else {
                cpu.inc_pc();
            }
        },
        sixteen_bits_Shift{rd, ..} => {
            if matches!(rd, DecodedRegister::PC) {
                unimplemented!();//TODO what is the behaviour in this case?
            } else {
                cpu.inc_pc();
            }
        },
        Base_plus_Disp6{rd, ..} => {
            if matches!(rd, DecodedRegister::PC) {
                unimplemented!();//TODO what is the behaviour in this case?
            } else {
                cpu.inc_pc();
            }
        },
        IMM6{rd, ..} => {
            if matches!(rd, DecodedRegister::PC) {
                unimplemented!();//TODO what is the behaviour in this case?
            } else {
                cpu.inc_pc();
            }
        },
        Stack_Operation{rd_index, ..} => {
            if *rd_index == 0b111 {//PC is the highest index, so we don't need to worry about size
                unimplemented!();//TODO what is the behaviour in this case?
            } else {
                cpu.inc_pc();
            }
        },
        DS_Indirect{rd, ..} => {
            if matches!(rd, DecodedRegister::PC) {
                unimplemented!();//TODO what is the behaviour in this case?
            } else {
                cpu.inc_pc();
            }
        },
        IMM16{rd, ..} => {
            if matches!(rd, DecodedRegister::PC) {
                unimplemented!();//TODO what is the behaviour in this case?
            } else {
                cpu.inc_pc_by(2);
            }
        },
        Direct16{rd, ..} => {
            if matches!(rd, DecodedRegister::PC) {
                unimplemented!();//TODO what is the behaviour in this case?
            } else {
                cpu.inc_pc_by(2);
            }
        },
        Direct6{rd, ..} => {
            if matches!(rd, DecodedRegister::PC) {
                unimplemented!();//TODO what is the behaviour in this case?
            } else {
                cpu.inc_pc();
            }
        },
        Register{rd, ..} => {
            if matches!(rd, DecodedRegister::PC) {
                unimplemented!();//TODO what is the behaviour in this case?
            } else {
                cpu.inc_pc();
            }
        },

        Invalid => { debug_panic!(); }//TODO proper error handling?
    }
}
