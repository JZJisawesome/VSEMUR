/* unsp.rs
 * By: John Jekel
 *
 * Module for fetching, decoding, and executing unSP instructions, as well as handling interrupts
 *
*/

//TODO remove this once everything is implemented
#![allow(unused_variables)]
#![allow(dead_code)]

/* Imports */

mod alu;
//mod bitop;
mod control;
//mod muldiv;
mod stack;
//mod shift16;

use crate::debug_panic;

use crate::logging::log;
use crate::logging::log_noln;
use crate::logging::log_finln;

use crate::interpreter::common::CPU;
use crate::interpreter::common::InstructionMemory;
use crate::interpreter::common::ReadableMemory;
use crate::interpreter::common::WritableMemory;
use crate::interpreter::common::InterruptClearable;
use crate::interpreter::common::InterruptReadable;
use crate::interpreter::common::Interrupt;

use crate::decode::*;//TODO only import what is needed from here

use super::common::inc_page_addr_by;

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

pub(super) fn emulate_inst(state: &mut (impl CPU + InstructionMemory + ReadableMemory + WritableMemory + InterruptClearable)) -> u8 {//Return the number of cycles the instruction would take to execute
    //Fetch instruction from memory
    debug_assert!(state.get_cs() < 0b111111);
    log!(1, "unSP: Fetch started from CS page, PC address: {:#04X}_{:04X}", state.get_cs(), state.get_pc());
    let inst_word: u16 = state.fetch_page_addr(state.get_cs(), state.get_pc());
    log!(2, "Instruction word group 1: {:#06X} | {:#018b}", inst_word, inst_word);

    //Decode it
    let mut decoded_inst = DecodedInstruction::Invalid;
    decode_wg1(inst_word, &mut decoded_inst);
    if needs_decode_wg2(&decoded_inst) {
        log!(1, "unSP: Fetch started from CS page, PC address + 1");
        let address_after_pc_tuple = inc_page_addr_by(state.get_cs(), state.get_pc(), 1);
        let wg2 = state.fetch_page_addr(address_after_pc_tuple.0, address_after_pc_tuple.1);
        log!(2, "Instruction word group 2: {:#06X} | {:#018b}", wg2, wg2);
        decode_wg2(&mut decoded_inst, wg2);
    }

    let cycles_executed = execute_inst(state, &decoded_inst);
    log!(1, "unSP: CS page, PC is now {:#04X}_{:04X} | SP is now {:#04X}", state.get_cs(), state.get_pc(), state.get_sp());
    log!(1, "unSP: Emulated one instruction in {} clock cycles", cycles_executed);
    return cycles_executed;
}

pub(super) fn handle_interrupts(state: &mut (impl CPU + ReadableMemory + InterruptReadable)) {
    //Check the state for new interrupts (using InterruptReadable), and if there is, push the current PC/SR/etc to the stack, read from the interrupt vector, and switch the PC to that location
    //TODO we need to not handle interrupts if we're currently in an interrupt handler
    log!(1, "unSP: Check for and handle interrupts");
    let interrupt_vector_addr: u16;
    use Interrupt::*;
    match state.get_interrupt() {
        Some(Break) => { todo!(); },
        Some(FIQ) => { todo!(); },
        Some(IRQ0) => { todo!(); },
        Some(IRQ1) => { todo!(); },
        Some(IRQ2) => { todo!(); },
        Some(IRQ3) => { todo!(); },
        Some(IRQ4) => { todo!(); },
        Some(IRQ5) => { todo!(); },
        Some(IRQ6) => { todo!(); },
        Some(IRQ7) => { todo!(); },
        None => { log!(2, "No new interrupts to deal with"); return; },//No interrupt to handle
    }

    todo!();
}

fn execute_inst(state: &mut (impl CPU + ReadableMemory + WritableMemory + InterruptClearable), inst: &DecodedInstruction) -> u8 {
    log!(1, "unSP: Execute instruction");

    use DecodedInstruction::*;
    match inst {
        Base_plus_Disp6{..} | IMM6{..} | DS_Indirect{..} | IMM16{..} | Direct16{..} | Direct6{..} | Register{..} => {
            return alu::execute(state, inst);
        },
        Register_BITOP_Rs{..} | Register_BITOP_offset{..} | Memory_BITOP_offset{..} | Memory_BITOP_Rs{..} => {
            todo!();//bitop::execute(state, inst);
        },
        CALL{..} | JMPF{..} | JMPR{..} | BREAK{..} | CALLR{..} | RETI{..} | RETF{..} | Branch{..} => {
            return control::execute(state, inst);
        },
        DIVS{..} | DIVQ{..} | EXP{..} | MUL{..} | MULS{..} => {
            todo!();//muldiv::execute(state, inst);
        },
        sixteen_bits_Shift{rd, op, rs} => {
            todo!();//shift16::execute(state, *rd, *op, *rs);
        },
        Stack_Operation{op, rd_index, size, rs} => {
            return stack::execute(state, *op, *rd_index, *size, *rs);
        },
        DSI6{imm6} => {
            state.set_ds(*imm6);
            state.inc_pc();
            return 2;
        },
        FIR_MOV{fir}=> {
            state.set_fir(*fir);
            state.inc_pc();
            return 2;
        },
        Fraction{fra} => {
            state.set_fra(*fra);
            state.inc_pc();
            return 2;
        },
        INT_SET{f, i} => {
            state.set_fiq(*f);
            state.set_irq(*i);
            state.inc_pc();
            return 2;
        },
        IRQ{i} => {
            state.set_irq(*i);
            state.inc_pc();
            return 2;
        },
        SECBANK{s} => {
            state.set_bnk(*s);
            state.inc_pc();
            return 2;
        },
        FIQ{f} => {
            state.set_fiq(*f);
            state.inc_pc();
            return 2;
        },
        IRQ_Nest_Mode{n} => {
            state.set_ine(*n);
            state.inc_pc();
            return 2;
        },
        NOP => {
            //We don't need to do anything! :)
            state.inc_pc();
            return 2;
        },
        DS_Access{w, rs} => {
            unimplemented!();//TODO do here
            state.inc_pc();
            return 2;
        },
        FR_Access{w, rs} => {
            unimplemented!();//TODO do here
            state.inc_pc();
            return 2;
        },

        Invalid => { return debug_panic!(0); }//TODO proper error handling?
    }
}
