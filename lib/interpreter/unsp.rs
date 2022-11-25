/* NAME//TODO
 * By: John Jekel
 *
 * TODO description
 *
*/

/* Imports */

use crate::debug_panic;

use crate::logging::log;

use crate::interpreter::common::CPU;
use crate::interpreter::common::InstructionMemory;
use crate::interpreter::common::ReadableMemory;
use crate::interpreter::common::WritableMemory;
use crate::interpreter::common::InterruptClearable;
use crate::interpreter::common::InterruptReadable;

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

fn emulate_inst(state: &mut (impl CPU + InstructionMemory + ReadableMemory + WritableMemory + InterruptClearable)) -> u8 {//Return the number of cycles the instruction would take to execute
    todo!();
}

fn handle_interrupts(state: &mut (impl CPU + ReadableMemory + InterruptReadable)) {
    //Check the state for new interrupts (using InterruptReadable), and if there is, push the current PC/SR/etc to the stack, read from the interrupt vector, and switch the PC to that location
    todo!();
}
