/* bitop.rs
 * By: John Jekel
 *
 * Emulates register and memory bitop instructions
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
use crate::logging::log_finln;
use crate::interpreter::common::CPU;
use crate::interpreter::common::ReadableMemory;
use crate::interpreter::common::WritableMemory;
use crate::decode::*;//TODO only import what is needed from here
use crate::decode::DecodedInstruction::*;

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
    match inst {
        //TODO others
        _ => { return debug_panic!(0); }//We should not have recieved this type of instruction
    }
}
