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

macro_rules! rd_index {
    ($inst_word:expr) => {
        (($inst_word >> 9) & 0b111) as u8
    };
}
pub(crate) use rd_index;//FIXME prevent having to export these helper macros to the whole crate (limitation of rust)

macro_rules! rs_index {
    ($inst_word:expr) => {
        ($inst_word & 0b111) as u8
    };
}
pub(crate) use rs_index;//FIXME prevent having to export these helper macros to the whole crate (limitation of rust)

macro_rules! imm6 {
    ($inst_word:expr) => {
        ($inst_word & 0b111111) as u8
    };
}
pub(crate) use imm6;//FIXME prevent having to export these helper macros to the whole crate (limitation of rust)

macro_rules! upper_nibble {
    ($inst_word:expr) => {
        $inst_word >> 12
    };
}
pub(crate) use upper_nibble;//FIXME prevent having to export these helper macros to the whole crate (limitation of rust)

macro_rules! secondary_group {
    ($inst_word:expr) => {
        ($inst_word >> 6) & 0b111
    };
}
pub(crate) use secondary_group;//FIXME prevent having to export these helper macros to the whole crate (limitation of rust)

macro_rules! reg_string_by_index {
    ($rs:expr) => {{
        debug_assert!($rs < 8);
        let string: &str;
        match $rs  {
            0b000 => { string = "SP"; },
            0b001 => { string = "R1"; },
            0b010 => { string = "R2"; },
            0b011 => { string = "R3"; },
            0b100 => { string = "R4"; },
            0b101 => { string = "BP"; },
            0b110 => { string = "SR"; },
            0b111 => { string = "PC"; },
            _ => { panic!(); },//This should never occur
        }
        string
    }};
}
pub(crate) use reg_string_by_index;//FIXME prevent having to export these helper macros to the whole crate (limitation of rust)

macro_rules! log_register {
    ($indent:expr, $reg_name:expr, $reg_index:expr) => {
        log!($indent, "{} is {:#05b}, aka {}", $reg_name, $reg_index, crate::interpreter::cpu::execute::reg_string_by_index!($reg_index));
    };
    ($indent:expr, $reg_name:expr, $reg_index:expr, $reg_contents:expr) => {
        log!($indent, "{} is {:#05b}, aka {}, which contains:", $reg_name, $reg_index, crate::interpreter::cpu::execute::reg_string_by_index!($reg_index));
        log!($indent + 1, "{:#06X} | {:#018b} | unsigned {}", $reg_contents, $reg_contents, $reg_contents);
    };
}
pub(crate) use log_register;//FIXME prevent having to export these helper macros to the whole crate (limitation of rust)

/* Static Variables */

//TODO

/* Types */

//TODO

/* Associated Functions and Methods */

//TODO

/* Functions */

pub(super) fn execute(cpu: &mut CPUState, mem: &mut MemoryState, inst_word: u16) {
    debug_assert!(mem.ready());

    let upper_nibble = upper_nibble!(inst_word);
    let secondary_group = secondary_group!(inst_word);
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

fn get_reg_by_index(cpu: &CPUState, rs: u8) -> u16 {
    debug_assert!(rs < 8);
    match rs {
        0b000 => {
            return (cpu.sp & 0xFFFF) as u16;
        },
        0b001 => {
            return cpu.r[0];
        },
        0b010 => {
            return cpu.r[1];
        },
        0b011 => {
            return cpu.r[2];
        },
        0b100 => {
            return cpu.r[3];
        },
        0b101 => {
            return cpu.bp;
        },
        0b110 => {
            return cpu.sr;
        },
        0b111 => {
            return cpu.pc;
        },
        _ => { panic!(); },//This should never occur
    }
}
fn set_reg_by_index(cpu: &mut CPUState, rd: u8, value: u16) {
    debug_assert!(rd < 8);
    match rd {
        0b000 => {
            cpu.sp = value;
        },
        0b001 => {
            cpu.r[0] = value;
        },
        0b010 => {
            cpu.r[1] = value;
        },
        0b011 => {
            cpu.r[2] = value;
        },
        0b100 => {
            cpu.r[3] = value;
        },
        0b101 => {
            cpu.bp = value;
        },
        0b110 => {
            cpu.sr = value;
        },
        0b111 => {
            cpu.pc = value;
        },
        _ => { panic!(); },//This should never occur
    }
}
