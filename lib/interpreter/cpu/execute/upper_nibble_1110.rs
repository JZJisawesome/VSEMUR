/* NAME//TODO
 * By: John Jekel
 *
 * TODO description
 *
*/

//TODO remove this once everything is implemented
#![allow(unused_variables)]
#![allow(dead_code)]
#[allow(unused_imports)]

/* Imports */

use crate::logging::*;
use crate::interpreter::memory::MemoryState;
use super::CPUState;

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

pub(super) fn execute(t: u32, cpu: &mut CPUState, mem: &mut MemoryState, inst_word: u16) {
    let secondary_group = (inst_word >> 6) & 0b111;
    debug_assert!(secondary_group < 8);

    unimplemented!();//TODO
}
