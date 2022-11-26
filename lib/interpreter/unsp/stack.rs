/* stack.rs
 * By: John Jekel
 *
 * Emulates Stack Operation instructions
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
use crate::decode::DecodedStackOp::*;

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

pub(super) fn execute(state: &mut (impl CPU + ReadableMemory + WritableMemory), op: DecodedStackOp, rd_index: u8, size: u8, rs: DecodedRegister) -> u8 {
    //The ISA docs are really confusing about how to properly interpret the rd/rh and size fields, and the direction to access the registers in
    //Looking at MAME clears things up. PUSH pushes from rd_index + 1 - size to rd_index (both inclusive) downwards
    //POP pops from rd_index + 1 to rd_index + size (both inclusive) upwards

    //TODO sanity check the size

    let mut new_rs: u16 = state.get_reg(rs);
    log!(3, "Rs currently contains {:#06X}", new_rs);

    //HACK We assume the SP will always point to page 0 (where memory is on the vsmile), so we never update the ds register here for speed
    match op {
        PUSH => {
            let min_index = rd_index;
            let max_index = rd_index + 1 - size;
            log!(3, "Begin push loop from {} to {} inclusive downwards", max_index, min_index);
            for i in max_index..=min_index {
                let data = state.get_reg_by_index(i);
                log!(4, "Push register with index {0:#05b}, containing {1:#04X} | {1:#08b} | unsigned {1}, to address 0x00_{2:04X}", i, data, new_rs);
                state.write_page_addr(0x00, new_rs, data);
                new_rs -= 1;
            }
        },
        POP => {
            let min_index = rd_index + 1;
            let max_index = rd_index + size;
            log!(3, "Begin pop loop from {} to {} inclusive upwards", min_index, max_index);
            for i in min_index..=max_index {
                new_rs += 1;
                let data = state.read_page_addr(0x00, new_rs);
                log!(4, "Pop from address 0x00_{2:04X}, containing {1:#04X} | {1:#08b} | unsigned {1}, to register with index {0:#05b}", new_rs, data, i);
                state.set_reg_by_index(i, data);
            }
        },

        Invalid => { debug_panic!(); }
    }

    state.set_reg(rs, new_rs);
    log!(3, "Rs now contains {:#06X}", new_rs);

    state.inc_pc();//FIXME what if the PC is one of the registers popped?
    return (2 * size) + 4;
}
