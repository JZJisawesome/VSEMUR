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

pub(super) fn fetch(state: &State) -> Inst {

    let pc_byte: u16 = 245;//TODO
    log!(state.t, 1, "Fetch started from PC={:#06X}", pc_byte);

    return Inst{wg: [0, 0]};//TODO
}
