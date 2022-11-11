/* NAME//TODO
 * By: John Jekel
 *
 * TODO description
 *
*/

/* Imports */

use crate::logging::log;

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
        log!(0, 1, "Initializizing Sound State");

        //TODO implement
        return SoundState {};
        //unimplemented!();
    }

    pub(super) fn reset(self: &mut Self) {
        log!(0, 1, "Resetting sound system");
        //unimplemented!();//TODO
    }

    pub fn tick(self: &mut Self) {
        //unimplemented!();//TODO
    }
}

/* Functions */

//TODO
