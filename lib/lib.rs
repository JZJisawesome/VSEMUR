/* libvsemur
 * By: John Jekel
 *
 * Library for the VSmile EMUlator in Rust
 *
 * Note: This library may not report errors to your application if it is compiled in release mode for speed.
 * If you are developing an application, you should compile this library in debug mode in order to be guaranteed to get errors back.
 *
*/

//!libvsemur
//!
//!By: John Jekel
//!
//!Library for the VSmile EMUlator in Rust

/* Imports */

pub mod about;//We want to export the module itself, into the root namespace, not it's contents directly, so we don't use ::*
pub mod interpreter;
pub mod jit;
pub mod decode;
mod logging;

/* Constants */

//TODO

/* Macros */

macro_rules! debug_panic {
    () => {
        if cfg!(debug_assertions) {
            panic!();
        }
    };
    ($release_value:expr) => {
        if cfg!(debug_assertions) {
            panic!();
        } else {
            $release_value
        }
    };
}
pub(crate) use debug_panic;

/* Static Variables */

//TODO

/* Types */

//TODO

/* Associated Functions and Methods */

//TODO

/* Functions */

//TODO

