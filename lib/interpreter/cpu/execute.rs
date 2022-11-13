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

mod upper_nibble_1111;
mod upper_nibble_1110;
mod other_upper_nibbles;

use crate::logging::log;
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
    debug_assert!(mem.ready());

    let upper_nibble = inst_word >> 12;
    let secondary_group = (inst_word >> 6) & 0b111;
    debug_assert!(upper_nibble < 16);
    debug_assert!(secondary_group < 8);
    log!(1, "CPU: Execute instruction of the form:    {:#06b}xxx{:03b}xxxxxx", upper_nibble, secondary_group);

    if (inst_word == 0xFFFF) || (inst_word == 0x0000) {//All zero or all one instructions are not valid
        log!(2, "Instruction type: (invalid)");
    }

    log!(2, "Check if upper nibble is 0xF or 0xE:     ^^^^");
    match upper_nibble {
        0xF => {
            log!(3, "Yep! Split opcodes by upper nibble     ^^^^");
            log!(3, "using secondary group and followed by         ^^^");
            log!(3, "other bits to narrow down instruction      ^^^   ^^^^^^");
            upper_nibble_1111::execute(cpu, mem, inst_word);
        },
        0xE => {
            log!(3, "Yep! Split opcodes by upper nibble     ^^^^");
            log!(3, "using secondary group and followed by         ^^^");
            log!(3, "other bits to narrow down instruction      ^^^   ^^^^^^");
            upper_nibble_1110::execute(cpu, mem, inst_word);
        },
        _ => {
            //For nibbles other than 0xF and 0xE, it is easier to decode the instruction by looking at the secondary group first
            //This is what MAME does, so it's what we'll do too
            log!(3, "Nope! Split opcodes by secondary group        ^^^");
            log!(3, "using upper nibble followed by         ^^^^");
            log!(3, "other bits to narrow down instruction      ^^^   ^^^^^^");
            other_upper_nibbles::execute(cpu, mem, inst_word);
        },
    }
}

//We may need the word after the current instruction (wordgroup 1)
fn get_wg2(cpu: &CPUState, mem: &mut MemoryState) -> u16 {
    let address_after_pc_tuple = super::inc_page_addr_by(cpu.get_cs(), cpu.pc, 1);
    return mem.read_page_addr(address_after_pc_tuple.0, address_after_pc_tuple.1);
}

fn push_sp(cpu: &mut CPUState, mem: &mut MemoryState, value: u16) {
    //HACK We assume the SP will always point to page 0 (where memory is on the vsmile), so we never update the ds register here for speed
    mem.write_page_addr(value, 0x00, cpu.sp);
    cpu.sp -= 1;
}

fn pop_sp(cpu: &mut CPUState, mem: &MemoryState) -> u16 {
    //HACK We assume the SP will always point to page 0 (where memory is on the vsmile), so we never update the ds register here for speed
    cpu.sp += 1;
    return mem.read_page_addr(0x00, cpu.sp);
}
