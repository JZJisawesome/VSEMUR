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

    log!(state.t, 1, "Fetch started from PC={:#08X}", state.regs.pc);//TODO seperate page from PC in printout
    debug_assert!(state.regs.pc < ((crate::interpreter::MEM_SIZE_BYTES as u32) - 1));//We need to fetch at least 2 bytes

    inst.wg[0] = ((state.mem[(state.regs.pc + 1) as usize] as u16) << 8) | (state.mem[state.regs.pc as usize] as u16);
    log!(state.t, 2, "Wordgroup 0: {:#06X} | {:#018b}", inst.wg[0], inst.wg[0]);

    if state.regs.pc < ((crate::interpreter::MEM_SIZE_BYTES as u32) - 3) {//There are another 2 bytes we can fetch (execute will decide if they're useful or not)
        inst.wg[1] = ((state.mem[(state.regs.pc + 3) as usize] as u16) << 8) | (state.mem[(state.regs.pc + 2) as usize] as u16);
        log!(state.t, 2, "Wordgroup 1: {:#06X} | {:#018b}", inst.wg[1], inst.wg[1]);
    }

    return true;
}
