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

    debug_assert!(state.regs.pc < ((crate::interpreter::MEM_SIZE_WORDS as u32) - 1));//We need to fetch at least 1 word
    log!(state.t, 1, "Fetch started from PC: {:#04X}_{:04X}", state.regs.pc >> 16, state.regs.pc & 0xFFFF);

    inst.wg[0] = state.mem[state.regs.pc as usize];
    log!(state.t, 2, "Wordgroup 0: {:#06X} | {:#018b}", inst.wg[0], inst.wg[0]);

    if state.regs.pc < ((crate::interpreter::MEM_SIZE_WORDS as u32) - 2) {//There is another word we can fetch (execute will decide if they're useful or not)
        inst.wg[1] = state.mem[(state.regs.pc + 1) as usize];
        log!(state.t, 2, "Wordgroup 1: {:#06X} | {:#018b}", inst.wg[1], inst.wg[1]);
    }

    return true;
}
