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

pub(super) fn execute(cpu: &mut CPUState, mem: &mut MemoryState, inst_word: u16) {
    log_noln!(4, "Instruction type: ");
    if ((inst_word >> 8) & 0b1) == 0b1 {//Part of the secondary group
        register_bitop(cpu, mem, inst_word);
    } else {
        memory_bitop(cpu, mem, inst_word);
    }
}

pub(super) fn register_bitop(cpu: &mut CPUState, mem: &mut MemoryState, inst_word: u16) {
    log_midln!("Register BITOP, ");

    if ((inst_word >> 6) & 0b1) == 0b1 {//Part of the secondary group
        log_finln!("offset");
        unimplemented!();//TODO
    } else {
        log_finln!("Rs");
        unimplemented!();//TODO
    }
}

pub(super) fn memory_bitop(cpu: &mut CPUState, mem: &mut MemoryState, inst_word: u16) {
    log_midln!("Memory BITOP, ");

    if ((inst_word >> 7) & 0b1) == 0b1 {//Part of the secondary group
        log_finln!("offset");
        unimplemented!();//TODO
    } else {
        log_finln!("Rs");
        unimplemented!();//TODO
    }
}
