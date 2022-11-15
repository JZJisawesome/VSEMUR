/* NAME//TODO
 * By: John Jekel
 *
 * TODO description
 *
*/

/* Imports */

use crate::interpreter::cpu::decode::DecodedInstruction;
use crate::interpreter::cpu::decode::DecodedInstruction::*;

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

pub(super) fn disassemble(decoded_inst: &DecodedInstruction) -> String {//TODO expose this from the library in some good way and create a vsemur-disassemble crate
    let assembly: String = "TODO".to_string();
    return assembly;
}
