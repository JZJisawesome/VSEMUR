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
            //Push the current PC, followed by the current SR, to the stack
            log!(3, "Push the current PC, follow by the current SR, to the stack");
            //HACK We assume the SP will always point to page 0 (where memory is on the vsmile), so we never update the ds register here for speed
            mem.write_page_addr(cpu.pc, 0x00, cpu.sp);
            cpu.sp -= 1;
            mem.write_page_addr(cpu.sr, 0x00, cpu.sp);
            cpu.sp -= 1;

            //Update the CS (which is contained within SR) and the PC
            log!(3, "The CS becomes the high 6 bits of A22, and the PC becomes the low 16 bits");
            cpu.set_cs(((a22 >> 16) & 0b111111) as u8);
            cpu.pc = (a22 & 0xFFFF) as u16;
        },
        //TODO others
        _ => { debug_panic!(); },//We should not have recieved this type of instruction
    }
}
