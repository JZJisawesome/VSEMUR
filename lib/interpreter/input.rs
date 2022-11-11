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

pub(super) struct InputState {
    p1: ControllerButtons,
    p2: ControllerButtons,
}


struct ControllerButtons {
    red: bool,
    yellow: bool,
    blue: bool,
    green: bool,

    enter: bool,
    help: bool,
    exit: bool,
    abc: bool,

    up: bool,
    down: bool,
    left: bool,
    right: bool,

    //TODO figure out how to handle the touchpads if they exist
}

/* Associated Functions and Methods */

impl InputState {
    pub(super) fn new() -> InputState {
        log!(0, 1, "Initializizing Input State");

        return InputState {
            p1: ControllerButtons {
                red: false,
                yellow: false,
                blue: false,
                green: false,

                enter: false,
                help: false,
                exit: false,
                abc: false,

                up: false,
                down: false,
                left: false,
                right: false,
            },
            p2: ControllerButtons {
                red: false,
                yellow: false,
                blue: false,
                green: false,

                enter: false,
                help: false,
                exit: false,
                abc: false,

                up: false,
                down: false,
                left: false,
                right: false,
            },
        };
    }

    pub(super) fn reset(self: &mut Self) {
        log!(0, 1, "Resetting input state");
        //unimplemented!();//TODO
    }
}

/* Functions */

//TODO
