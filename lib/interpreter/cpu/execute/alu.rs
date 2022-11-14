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
    unimplemented!();//TODO
    match inst {
        //TODO others
        _ => { panic!(); }//We should not have recieved this type of instruction
    }
}
