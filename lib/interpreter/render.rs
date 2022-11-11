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

pub(super) struct RenderState {
    //TODO
}

/* Associated Functions and Methods */

impl RenderState {
    pub(super) fn new() -> RenderState {
        log!(0, 1, "Initializizing Render State");

        //TODO implement
        return RenderState {};
        //unimplemented!();
    }

    pub(super) fn reset(self: &mut Self) {
        log!(0, 1, "Resetting render logic");
        //unimplemented!();//TODO
    }

    pub fn tick(self: &mut Self) {
        //unimplemented!();//TODO
    }
}

/* Functions */

//TODO
