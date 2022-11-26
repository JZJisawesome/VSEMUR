/* vsemur-cli
 * Copyright (C) 2022 John Jekel
 * See the LICENSE file at the root of the project for licensing info.
 *
 * libvsemur command-line frontend
 *
*/

//!libvsemur command-line frontend

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

    //Initialize emulator
    let mut emulator = vsemur::interpreter::Emulator::new();
    if !matches!(emulator.load_bios_file(&std::env::args().nth(1).unwrap()), Ok(())) {
        eprintln!("\x1b[31mError: Failed to load bios from disk\x1b[0m\n");
        return;
    }
    if !matches!(emulator.load_rom_file(&std::env::args().nth(2).unwrap()), Ok(())) {
        eprintln!("\x1b[31mError: Failed to load rom from disk\x1b[0m\n");
        return;
    }
    emulator.reset();//Power-on reset AFTER loading bios and rom
    //TODO other setup (channels)

    //TESTING
    debug_assert!(!emulator.thread_running());
    eprintln!("started");
    emulator.launch_emulation_thread();
    debug_assert!(emulator.thread_running());
    eprintln!("waiting");
    std::thread::sleep(std::time::Duration::from_millis(5000));
    eprintln!("stopping");
    emulator.stop_emulation_thread();
    eprintln!("stopped");
    debug_assert!(!emulator.thread_running());
}
