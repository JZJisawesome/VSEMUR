/* memory.rs
 * By: John Jekel
 *
 * Memory access functions for VSmile emulation
 *
*/

/* Imports */

use crate::logging::log;
use crate::interpreter::State;
use crate::interpreter::Inst;

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

pub(super) fn fetch(state: &State, inst: &mut Inst) -> bool {
    debug_assert!(state.mem_loaded);

    let pc_bytes: u32 = (state.regs.pc * 2) as u32;

    log!(state.t, 1, "Fetch started from PC={:#06X}, aka byte {:#07X}", state.regs.pc, pc_bytes);
    debug_assert!(pc_bytes < ((crate::interpreter::MEM_SIZE_BYTES as u32) - 1));//We need to fetch at least 2 bytes

    inst.wg[0] = ((state.mem[(pc_bytes + 1) as usize] as u16) << 8) | (state.mem[pc_bytes as usize] as u16);
    log!(state.t, 2, "Wordgroup 0={:#06X}", inst.wg[0]);

    if pc_bytes < ((crate::interpreter::MEM_SIZE_BYTES as u32) - 3) {//There are another 2 bytes we can fetch (execute will decide if they're useful or not)
        inst.wg[1] = ((state.mem[(pc_bytes + 3) as usize] as u16) << 8) | (state.mem[(pc_bytes + 2) as usize] as u16);
        log!(state.t, 2, "Wordgroup 1={:#06X}", inst.wg[1]);
    }

    return true;
}
