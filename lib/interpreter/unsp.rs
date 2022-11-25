/* unsp.rs
 * By: John Jekel
 *
 * Module for fetching, decoding, and executing unSP instructions and performing other operations
 *
*/

//TODO remove this once everything is implemented
#![allow(unused_variables)]
#![allow(dead_code)]

/* Imports */

/*mod alu;
mod bitop;
mod control;
mod muldiv;
mod stack;
mod shift16;*/

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

use crate::decode::*;//TODO only import what is needed from here
use crate::decode::DecodedInstruction::*;

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

    todo!();
}

pub(super) fn handle_interrupts(state: &mut (impl CPU + ReadableMemory + InterruptReadable)) {
    //Check the state for new interrupts (using InterruptReadable), and if there is, push the current PC/SR/etc to the stack, read from the interrupt vector, and switch the PC to that location
    todo!();
}
