/* control.rs
 * By: John Jekel
 *
 * Emulates the control-transfer instructions of the CPU
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
        CALL{a22} => {
            let new_page = (a22 >> 16) & 0b111111;
            let new_addr = a22 & 0xFFFF;
            unimplemented!();//TODO
        },
        _ => { debug_panic!(); },//We should not have recieved this type of instruction
    }
}
