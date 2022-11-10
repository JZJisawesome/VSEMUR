/* execute.rs
 * By: John Jekel
 *
 * Module for executing VSmile instructions and performing other operations
 *
*/

/* Imports */

mod upper_nibble_1111;
mod upper_nibble_1110;
mod other_upper_nibbles;

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

pub(super) fn execute(state: &mut State, inst: &Inst) -> bool {
    debug_assert!(state.mem_loaded);

    let upper_nibble = inst.wg[0] >> 12;
    let secondary_group = (inst.wg[0] >> 6) & 0b111;
    debug_assert!(upper_nibble < 16);
    debug_assert!(secondary_group < 8);
    log!(state.t, 1, "Execute started on instruction type: {:#06b}xxx{:03b}xxxxxx", upper_nibble, secondary_group);

    if (inst.wg[0] == 0xFFFF) || (inst.wg[0] == 0x0000) {//All zero or all one instructions are not valid
        log!(state.t, 2, "Instruction: (invalid)");
        return true;
    }

    match upper_nibble {
        0xF => {
            upper_nibble_1111::execute(state, inst, secondary_group as u8);
        },
        0xE => {
            upper_nibble_1110::execute(state, inst, secondary_group as u8);
        },
        upper_nibble => {
            //For nibbles other than 0xF and 0xE, it is easier to decode the instruction by looking at the secondary group first
            //This is what MAME does, so it's what we'll do too
            other_upper_nibbles::execute(state, inst, upper_nibble as u8, secondary_group as u8);
        },
    }

    log!(state.t, 2, "PC is now {:#04X}_{:04X}", state.regs.pc >> 16, state.regs.pc & 0xFFFF);
    return true;
}

