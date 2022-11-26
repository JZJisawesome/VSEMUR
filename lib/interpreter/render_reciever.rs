/* render_reciever.rs
 * By: John Jekel
 *
 * Recieves RenderMessages from the emulation thread and renders frames in the user's thread for their code to consume
 *
*/

//TODO remove this once everything is implemented
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(dead_code)]

/* Imports */

//TODO (include "use" and "mod" here)

/* Constants */

//TODO

/* Macros */

//TODO (also pub(crate) use the_macro statements here too)

/* Static Variables */

//TODO

/* Types */

///TODO cargo doc for this
pub struct RenderReciever {//Public-facing struct for recieving RenderMessages

}

pub(super) struct RenderMessage {
    //TODO struct returned by a channel from the renderer containing the data/methods needed to render a frame or access the already rendered frame depending on how things go
}

/* Associated Functions and Methods */

//TODO

/* Functions */

//TODO
