/* vsemur-assemble
 * Copyright (C) 2022 John Jekel
 * See the LICENSE file at the root of the project for licensing info.
 *
 * Command-line frontend for exposed libvsemur assembler facilities
 *
*/

//!Command-line frontend for exposed libvsemur assembler facilities

/* Imports */

use std::io::Write;
use std::io::Read;

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

fn main() {
    //Print version info
    eprintln!("VSEMUR Assembler");
    eprintln!("Powered by: {}\n", vsemur::about::version::pretty_string());

    eprintln!("Hello, world! (Assembler)");//TODO other things here
}
