/* stack.rs
 * By: John Jekel
 *
 * Emulates Stack Operation instructions
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
use super::super::decode::DecodedStackOp::*;

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

pub(super) fn execute(cpu: &mut CPUState, mem: &mut MemoryState, op: DecodedStackOp, rd_index: u8, size: u8, rs: DecodedRegister) {
    debug_assert!(size <= (rd_index + 1));
    if size == 0 {
        log!(3, "Size is 0, so we're not doing anything");
        return;
    }
    let min_index = (rd_index + 1) - size;//Inclusive

    let mut new_rs: u16 = cpu.get_reg(rs);
    log!(3, "Rs currently contains {:#06X}", new_rs);

    //HACK We assume the SP will always point to page 0 (where memory is on the vsmile), so we never update the ds register here for speed
    match op {
        PUSH => {
            log!(3, "Begin push loop");
            for i in rd_index..=min_index {
                let data = cpu.get_reg_by_index(i);
                log!(4, "Push register with index {0:#05b}, containing {1:#04X} | {1:#08b} | unsigned {1}, to address 0x00_{2:04X}", i, data, new_rs);
                mem.write_page_addr(data, 0x00, new_rs);
                new_rs -= 1;
            }
        },
        POP => {
            log!(3, "Begin pop loop");
            unimplemented!();//TODO
        },

        Invalid => { debug_panic!(); }
    }

    cpu.set_reg(rs, new_rs);
    log!(3, "Rs now contains {:#06X}", new_rs);
}
