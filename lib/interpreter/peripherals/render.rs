/* NAME//TODO
 * By: John Jekel
 *
 * TODO description
 *
*/

//TODO remove this once everything is implemented
#![allow(unused_variables)]
#![allow(dead_code)]
#[allow(unused_imports)]

/* Imports */

use crate::logging::log;

/* Constants */

//TODO

/* Macros */

//TODO (also pub(crate) use the_macro statements here too)

/* Static Variables */

//TODO

/* Types */

pub(super) struct RenderState {
    //TODO
}

/* Associated Functions and Methods */

impl RenderState {
    pub(super) fn new() -> RenderState {
        log!(1, "Initializing render state");

        //TODO implement
        return RenderState {};
        //unimplemented!();
    }

    pub(super) fn reset(self: &mut Self) {
        log!(1, "Resetting render state");
        //TODO zero out registers in mem
        //unimplemented!();//TODO
    }

    pub fn tick(self: &mut Self) {
        //unimplemented!();//TODO
        //TESTING
        //log!(1, "Printing render register contents");
        //for i in 0x002800..=0x0028FF {
        //    log!(2, "{:#08X}: {:#06X}", i, mem.read_addr(i));
        //}
    }
}

/* Functions */

//TODO
