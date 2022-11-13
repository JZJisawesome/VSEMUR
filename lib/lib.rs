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
mod logging;

/* Constants */

//TODO

/* Macros */

//TODO (also pub(crate) use the_macro statements here too)

/* Static Variables */

//TODO

/* Types */

//TODO

/* Associated Functions and Methods */

//TODO

/* Functions */

//TODO

