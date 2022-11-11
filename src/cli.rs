/* vsemur-cli
 * By: John Jekel
 *
 * libvsemur command-line frontend
 *
*/

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
    eprintln!("VSEMUR Command-Line Interface");
    eprintln!("Powered by: {}\n", vsemur::about::version::pretty_string());

    //Handle command line arguments
    let arg: String;
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
            eprintln!("\x1b[31mError: Expected 1 or 2 arguments (path to bios, path to rom; or --version)\x1b[0m\n");
            return;
        },
    }

    //Initialize state and load bios and rom
    let mut state: vsemur::interpreter::State = vsemur::interpreter::State::new();
    if !matches!(state.load_bios_file(&std::env::args().nth(1).unwrap()), vsemur::interpreter::ReturnCode::LOAD_OK) {
        eprintln!("\x1b[31mError: Failed to load bios from disk\x1b[0m\n");
        return;
    }
    if !matches!(state.load_rom_file(&std::env::args().nth(2).unwrap()), vsemur::interpreter::ReturnCode::LOAD_OK) {
        eprintln!("\x1b[31mError: Failed to load rom from disk\x1b[0m\n");
        return;
    }

    //Power-on reset
    let reset_result = state.reset();
    debug_assert!(matches!(reset_result, vsemur::interpreter::ReturnCode::RESET_OK));

    //Main emulation loop
    loop {
        match state.tick() {
            vsemur::interpreter::ReturnCode::TICK_OK => { continue; },
            vsemur::interpreter::ReturnCode::TICK_FAIL => {//This should never occur
                if cfg!(debug_assertions) {
                    panic!("\x1b[31mError: Tick failed\x1b[0m");
                }
            },
            _ => {
                if cfg!(debug_assertions) {
                    panic!("\x1b[31mError: Tick returned invalid code\x1b[0m");
                }
            },
        }
    }
}
