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
    log!(state.t, 1, "Execute started on instruction of form:  {:#06b}xxx{:03b}xxxxxx", upper_nibble, secondary_group);

    if (inst.wg[0] == 0xFFFF) || (inst.wg[0] == 0x0000) {//All zero or all one instructions are not valid
        log!(state.t, 2, "Instruction type: (invalid)");
        return true;
    }

    log!(state.t, 2, "Check if upper nibble is 0xF or 0xE:     ^^^^");
    match upper_nibble {
        0xF => {
            log!(state.t, 3, "Yep! Split opcodes by upper nibble     ^^^^");
            log!(state.t, 3, "using secondary group and followed by         ^^^");
            log!(state.t, 3, "other bits to narrow down instruction      ^^^   ^^^^^^");
            upper_nibble_1111::execute(state, inst, secondary_group as u8);
        },
        0xE => {
            log!(state.t, 3, "Yep! Split opcodes by upper nibble     ^^^^");
            log!(state.t, 3, "using secondary group and followed by         ^^^");
            log!(state.t, 3, "other bits to narrow down instruction      ^^^   ^^^^^^");
            upper_nibble_1110::execute(state, inst, secondary_group as u8);
        },
        upper_nibble => {
            //For nibbles other than 0xF and 0xE, it is easier to decode the instruction by looking at the secondary group first
            //This is what MAME does, so it's what we'll do too
            log!(state.t, 3, "Nope! Split opcodes by secondary group        ^^^");
            log!(state.t, 3, "using upper nibble followed by         ^^^^");
            log!(state.t, 3, "other bits to narrow down instruction      ^^^   ^^^^^^");
            other_upper_nibbles::execute(state, inst, upper_nibble as u8, secondary_group as u8);
        },
    }

    log!(state.t, 2, "CS page, PC is now {:#04X}_{:04X}", state.regs.sr.cs, state.regs.pc);
    return true;
}

