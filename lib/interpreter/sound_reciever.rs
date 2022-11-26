/* sound_reciever.rs
 * By: John Jekel
 *
 * Recieves SoundMessages from the emulation thread and processes sound in the user's thread for their code to consume
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
pub struct SoundReciever {//Public-facing struct for recieving RenderMessages

}

pub(super) struct SoundMessage {
    //TODO struct returned by a channel from the renderer containing the data/methods indicating how to change the audio being output
}

/* Associated Functions and Methods */

//TODO

/* Functions */

//TODO
