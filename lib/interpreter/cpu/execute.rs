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
mod shift16;

use crate::debug_panic;
use crate::logging::log;
use crate::logging::log_noln;
use crate::logging::log_finln;
use crate::interpreter::memory::MemoryState;
use super::CPUState;
use crate::decode::*;//TODO only import what is needed from here
use crate::decode::DecodedInstruction::*;

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
    configure_cycle_count(cpu, inst);
    increment_pc(cpu, inst);
}

fn perform_instruction(cpu: &mut CPUState, mem: &mut MemoryState, inst: &DecodedInstruction) {
    log!(2, "Perform instruction operations");
    match inst {
        Base_plus_Disp6{..} | IMM6{..} | DS_Indirect{..} | IMM16{..} | Direct16{..} | Direct6{..} | Register{..} => {//TODO potentiallly move sixteen_bits_Shift elsewhere (shift16.rs)
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
        sixteen_bits_Shift{rd, op, rs} => { shift16::execute(cpu, mem, *rd, *op, *rs); },
        Stack_Operation{op, rd_index, size, rs} => { stack::execute(cpu, mem, *op, *rd_index, *size, *rs); },
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

fn configure_cycle_count(cpu: &mut CPUState, inst: &DecodedInstruction) {
    log!(2, "Determine clock cycles it will take to execute");
    match inst {
        //TODO combine into individual cases more efficiently (to make more readable)
        DSI6{..} => { cpu.set_cycle_count(2); },
        CALL{..} => { cpu.set_cycle_count(9); },
        JMPF{..} => { cpu.set_cycle_count(5); },
        JMPR{..} => { cpu.set_cycle_count(4); },
        FIR_MOV{..}=> { cpu.set_cycle_count(2); },
        Fraction{..} => { cpu.set_cycle_count(2); },
        INT_SET{..} => { cpu.set_cycle_count(2); },
        IRQ{..} => { cpu.set_cycle_count(2); },
        SECBANK{..} => { cpu.set_cycle_count(2); },
        FIQ{..} => { cpu.set_cycle_count(2); },
        IRQ_Nest_Mode{..} => { cpu.set_cycle_count(2); },
        BREAK{..} => { cpu.set_cycle_count(10); },
        CALLR{..} => { cpu.set_cycle_count(8); },
        DIVS{..} => { cpu.set_cycle_count(2); }
        DIVQ{..} => { cpu.set_cycle_count(3); }
        EXP{..} => { cpu.set_cycle_count(2); }
        NOP{..} => { cpu.set_cycle_count(2); },
        DS_Access{..} => { cpu.set_cycle_count(2); },
        FR_Access{..} => { cpu.set_cycle_count(2); },
        MUL{..} => { cpu.set_cycle_count(12); },
        MULS{..} => {
            unimplemented!();//TODO figure out what N means (Size?)
        },
        Register_BITOP_Rs{..} => { cpu.set_cycle_count(4); }
        Register_BITOP_offset{..} => { cpu.set_cycle_count(4); }
        Memory_BITOP_offset{..} => { cpu.set_cycle_count(7); }
        Memory_BITOP_Rs{..} => { cpu.set_cycle_count(7); }
        sixteen_bits_Shift{..} => { cpu.set_cycle_count(8); },
        RETI{..} => {
            unimplemented!();//TODO requires accessing CPU state
        },
        RETF{..} => { cpu.set_cycle_count(8); },
        Base_plus_Disp6{..} => { cpu.set_cycle_count(6); },
        IMM6{..} => { cpu.set_cycle_count(2); },
        Branch{..} => { /* Since this depends on whether the branch is taken or not, we decide this earlier, not here */ },
        Stack_Operation{size, ..} => { cpu.set_cycle_count((2 * size) + 4); },
        DS_Indirect{rd, ..} => {
            if matches!(rd, DecodedRegister::PC) {
                cpu.set_cycle_count(7);
            } else {
                cpu.set_cycle_count(6);
            }
        },
        IMM16{rd, ..} => {
            if matches!(rd, DecodedRegister::PC) {
                cpu.set_cycle_count(5);
            } else {
                cpu.set_cycle_count(4);
            }
        },
        Direct16{rd, ..} => {
            if matches!(rd, DecodedRegister::PC) {
                cpu.set_cycle_count(8);
            } else {
                cpu.set_cycle_count(7);
            }
        },
        Direct6{rd, ..} => {
            if matches!(rd, DecodedRegister::PC) {
                cpu.set_cycle_count(6);
            } else {
                cpu.set_cycle_count(5);
            }
        },
        Register{rd, ..} => {
            if matches!(rd, DecodedRegister::PC) {
                cpu.set_cycle_count(5);
            } else {
                cpu.set_cycle_count(3);
            }
        },

        Invalid => { debug_panic!(); }//TODO proper error handling?
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
                //The PC was updated by the operation, so we don't do anything here//TODO is this correct?
            } else {
                cpu.inc_pc();
            }
        },
        MULS{rd, ..} => {
            if matches!(rd, DecodedRegister::PC) {
                //The PC was updated by the operation, so we don't do anything here//TODO is this correct?
            } else {
                cpu.inc_pc();
            }
        },
        Register_BITOP_Rs{rd, ..} => {
            if matches!(rd, DecodedRegister::PC) {
                //The PC was updated by the operation, so we don't do anything here//TODO is this correct?
            } else {
                cpu.inc_pc();
            }
        },
        Register_BITOP_offset{rd, ..} => {
            if matches!(rd, DecodedRegister::PC) {
                //The PC was updated by the operation, so we don't do anything here//TODO is this correct?
            } else {
                cpu.inc_pc();
            }
        },
        Memory_BITOP_offset{rd, ..} => {
            if matches!(rd, DecodedRegister::PC) {
                //The PC was updated by the operation, so we don't do anything here//TODO is this correct?
            } else {
                cpu.inc_pc();
            }
        },
        Memory_BITOP_Rs{rd, ..} => {
            if matches!(rd, DecodedRegister::PC) {
                //The PC was updated by the operation, so we don't do anything here//TODO is this correct?
            } else {
                cpu.inc_pc();
            }
        },
        sixteen_bits_Shift{rd, ..} => {
            if matches!(rd, DecodedRegister::PC) {
                //The PC was updated by the operation, so we don't do anything here//TODO is this correct?
            } else {
                cpu.inc_pc();
            }
        },
        Base_plus_Disp6{rd, ..} => {
            if matches!(rd, DecodedRegister::PC) {
                //The PC was updated by the operation, so we don't do anything here//TODO is this correct?
            } else {
                cpu.inc_pc();
            }
        },
        IMM6{rd, ..} => {
            if matches!(rd, DecodedRegister::PC) {
                //The PC was updated by the operation, so we don't do anything here//TODO is this correct?
            } else {
                cpu.inc_pc();
            }
        },
        Stack_Operation{rd_index, ..} => {
            if *rd_index == 0b111 {//FIXME this is harder for POP; *rd_index may not be the PC but it could affect it; not needed for push
                unimplemented!();//TODO what is the behaviour in this case?
            } else {
                cpu.inc_pc();
            }
        },
        DS_Indirect{rd, ..} => {
            if matches!(rd, DecodedRegister::PC) {
                //The PC was updated by the operation, so we don't do anything here//TODO is this correct?
            } else {
                cpu.inc_pc();
            }
        },
        IMM16{rd, ..} => {
            if matches!(rd, DecodedRegister::PC) {
                //The PC was updated by the operation, so we don't do anything here//TODO is this correct?
            } else {
                cpu.inc_pc_by(2);
            }
        },
        Direct16{rd, ..} => {
            if matches!(rd, DecodedRegister::PC) {
                //The PC was updated by the operation, so we don't do anything here//TODO is this correct?
            } else {
                cpu.inc_pc_by(2);
            }
        },
        Direct6{rd, ..} => {
            if matches!(rd, DecodedRegister::PC) {
                //The PC was updated by the operation, so we don't do anything here//TODO is this correct?
            } else {
                cpu.inc_pc();
            }
        },
        Register{rd, ..} => {
            if matches!(rd, DecodedRegister::PC) {
                //The PC was updated by the operation, so we don't do anything here//TODO is this correct?
            } else {
                cpu.inc_pc();
            }
        },

        Invalid => { debug_panic!(); }//TODO proper error handling?
    }
}
