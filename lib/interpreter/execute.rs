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
    debug_assert!(upper_nibble < 16);
    log!(state.t, 1, "Execute started with upper nibble: {:#06b}", upper_nibble);

    match upper_nibble {
        0xF => {
            upper_nibble_1111::execute(state, inst);
        },
        0xE => {
            upper_nibble_1110::execute(state, inst);
        },
        nibble => {
            other_upper_nibbles::execute(state, inst, nibble as u8);
        },
    }

    return true;
}

