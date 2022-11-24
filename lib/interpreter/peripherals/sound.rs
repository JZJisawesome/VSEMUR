/* NAME//TODO
 * By: John Jekel
 *
 * TODO description
 *
*/

/* Imports */

use crate::logging::log;

use crate::interpreter::common::ReadableMemory;
use crate::interpreter::common::WritableMemory;

/* Constants */

//TODO

/* Macros */

//TODO (also pub(crate) use the_macro statements here too)

/* Static Variables */

//TODO

/* Types */

pub(super) struct SoundState {
    //TODO
}

/* Associated Functions and Methods */

impl SoundState {
    pub(super) fn new() -> SoundState {
        log!(2, "Initializing sound state");

        //TODO implement
        return SoundState {};
        //unimplemented!();
    }

    pub(super) fn reset(self: &mut Self) {
        log!(2, "Resetting sound state");
        //unimplemented!();//TODO
    }

    pub fn tick(self: &mut Self) {
        //unimplemented!();//TODO
    }
}

impl ReadableMemory for SoundState {
    fn read_addr(self: &Self, addr: u32) -> u16 {
        log!(2, "Sound Access");
        //todo!();
        return 0;//TODO implement sound (at least the registers so that the CPU gets the values it expects)
    }
}


impl WritableMemory for SoundState {
    fn write_addr(self: &mut Self, addr: u32, data: u16) {
        log!(2, "Sound Access");
        //todo!();//TODO implement sound
    }
}

/* Functions */

//TODO
