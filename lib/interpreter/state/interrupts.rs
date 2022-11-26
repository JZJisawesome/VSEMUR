/* NAME//TODO
 * Copyright (C) 2022 John Jekel
 * See the LICENSE file at the root of the project for licensing info.
 *
 * TODO description
 *
*/

/* Imports */

use super::State;

use crate::interpreter::common::Interrupt;
use crate::interpreter::common::InterruptReadable;
use crate::interpreter::common::InterruptClearable;

/* Constants */

//TODO

/* Macros */

//TODO (also pub(crate) use the_macro statements here too)

/* Static Variables */

//TODO

/* Types */

impl InterruptReadable for State {
    fn get_interrupt(self: &Self) -> Option<Interrupt> {
        todo!();
    }
}

impl InterruptClearable for State {
    fn clear_current_interrupt(self: &mut Self) {
        todo!();
    }
}

/* Associated Functions and Methods */

//TODO

/* Functions */

//TODO
