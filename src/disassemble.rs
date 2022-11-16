/* vsemur-disassemble
 * By: John Jekel
 *
 * Command-line frontend for exposed libvsemur disassembler facilities
 *
*/

//!Command-line frontend for exposed libvsemur disassembler facilities

/* Imports */

//TODO (include "use" and "mod" here)

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
    eprintln!("VSEMUR Disassembler");
    eprintln!("Powered by: {}\n", vsemur::about::version::pretty_string());

    //Handle command line arguments
    match std::env::args().len() {
        2 => {
            if std::env::args().nth(1).unwrap() == "--version" {
                eprintln!("{}", vsemur::about::LICENSE);
                return;
            } else {
                eprintln!("\x1b[31mError: Invalid argument\x1b[0m\n");
                return;
            }
        },
        3 => {},//Continue to the next part of main()
        _ => {
            eprintln!("\x1b[31mError: Expected 1 or 2 arguments (path to file to disassembly, path to output disassembly file; or --version)\x1b[0m\n");
            return;
        },
    }

    //Open files//TODO proper error handling
    let input_file_wrapper = std::fs::OpenOptions::new().read(true).open(std::env::args().nth(1).unwrap()).unwrap();
    let output_file_wrapper = std::fs::OpenOptions::new().append(true).write(true).create(true).open(std::env::args().nth(2).unwrap()).unwrap();

    eprintln!("Hello, world! (disassembler)");//TODO other things here
}
