/* muldiv.rs
 * By: John Jekel
 *
 * Emulates MUL, MULS, DIV, DIVQ, and EXP
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
use crate::interpreter::common::ReadableMemory;
use crate::interpreter::common::WritableMemory;
use super::CPUState;
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

pub(super) fn execute(cpu: &mut CPUState, mem: &mut (impl ReadableMemory + WritableMemory), inst: &DecodedInstruction) {
    match inst {
        MUL{s_rs, rd, s_rd, rs} => {
            match (s_rd, s_rs) {
                //TODO logging
                //This should be okay: https://web.mit.edu/rust-lang_v1.25/arch/amd64_ubuntu1404/share/doc/rust/html/book/first-edition/casting-between-types.html (heading Numeric Casts)
                (false, false) => {//unsigned*unsigned
                    let operand1: u32 = cpu.get_reg(*rd) as u32;
                    let operand2: u32 = cpu.get_reg(*rs) as u32;
                    let result: u32 = operand1 * operand2;
                    cpu.set_mr(result);
                },
                (true, false) => {//signed*unsigned
                    let operand1: i32 = (cpu.get_reg(*rd) as i16) as i32;//We want sign extension
                    let operand2: i32 = (cpu.get_reg(*rs) as u32) as i32;//We don't want sign extension
                    let result: i32 = operand1 * operand2;
                    cpu.set_mr(result as u32);
                },
                (true, true) => {//signed*signed
                    let operand1: i32 = (cpu.get_reg(*rd) as i16) as i32;//We want sign extension
                    let operand2: i32 = (cpu.get_reg(*rs) as i16) as i32;//We want sign extension
                    let result: i32 = operand1 * operand2;
                    cpu.set_mr(result as u32);
                },
                (_, _) => { debug_panic!(); }
            }
        },
        //TODO others
        _ => { debug_panic!(); }//We should not have recieved this type of instruction
    }
}
