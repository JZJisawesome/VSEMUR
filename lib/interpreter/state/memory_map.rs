/* NAME//TODO
 * By: John Jekel
 *
 * TODO description
 *
*/

/* Imports */

use super::State;

use crate::interpreter::common::InstructionMemory;
use crate::interpreter::common::ReadableMemory;
use crate::interpreter::common::WritableMemory;

/* Constants */

//TODO

/* Macros */

//TODO (also pub(crate) use the_macro statements here too)

/* Static Variables */

//TODO

/* Types */


impl InstructionMemory for State {
    fn should_invalidate_icache(self: &Self) -> bool {
        todo!();
    }

    fn fetch_addr(self: &Self, addr: u32) -> u16 {//For instruction fetching only (faster)
        todo!();
    }
}

impl ReadableMemory for State {
    fn read_addr(self: &Self, addr: u32) -> u16 {
        todo!();
    }
}

impl WritableMemory for State {
    fn write_addr(self: &mut Self, addr: u32, data: u16) {
        todo!();
    }
}

/* Associated Functions and Methods */

//TODO

/* Functions */

//TODO
