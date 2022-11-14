/* vsemur-cli
 * By: John Jekel
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

    //Initialize state and load bios and rom
    let mut state: vsemur::interpreter::State = vsemur::interpreter::State::new();
    if !matches!(state.load_bios_file(&std::env::args().nth(1).unwrap()), vsemur::interpreter::ReturnCode::LoadOk) {
        eprintln!("\x1b[31mError: Failed to load bios from disk\x1b[0m\n");
        return;
    }
    if !matches!(state.load_rom_file(&std::env::args().nth(2).unwrap()), vsemur::interpreter::ReturnCode::LoadOk) {
        eprintln!("\x1b[31mError: Failed to load rom from disk\x1b[0m\n");
        return;
    }

    //Power-on reset
    let reset_result = state.reset();
    debug_assert!(matches!(reset_result, vsemur::interpreter::ReturnCode::ResetOk));

    //Cache instructions//TESTING
    //state.cache();//TESTING

    //Main emulation loop
    let tick_period = std::time::Duration::from_nanos(37);//1/27Mhz is 0.00000003703 seconds, or 37.03703704ns//TODO perhaps move this to the library?
    let tick_leniency = std::time::Duration::from_nanos(10);//Most amount of time we tolarate the tick being late before displaying a warning
    let mut warning_displayed_flag: bool = false;//Only display the warning once

    let mut last_tick = std::time::Instant::now();
    loop {
        let time_since_last_tick = last_tick.elapsed();
        if time_since_last_tick >= tick_period {
            //Time to call state.tick();
            let tick_result = state.tick();
            //let tick_result = state.tick_cached();//TESTING
            last_tick = std::time::Instant::now();
            if !warning_displayed_flag && (time_since_last_tick > (tick_period + tick_leniency)) {
                eprintln!("\x1b[31mWarning: The last tick was too late ({} > {}), your system might be too slow to run VSEMUR at full speed...\x1b[0m", time_since_last_tick.as_nanos(), tick_period.as_nanos());
                warning_displayed_flag = true;
            }

            //Handle the tick's result
            match tick_result {
                vsemur::interpreter::ReturnCode::TickOk => { },
                vsemur::interpreter::ReturnCode::TickFail => {//This should never occur
                    if cfg!(debug_assertions) {
                        panic!("\x1b[31mError: Tick failed\x1b[0m");
                    }
                },
                vsemur::interpreter::ReturnCode::TickOkNewFrameAvailable => {
                    unimplemented!();//TODO implement
                }
                _ => {
                    if cfg!(debug_assertions) {
                        panic!("\x1b[31mError: Tick returned invalid code\x1b[0m");
                    }
                },
            }
        }

        //TODO rendering, etc here (perhaps set a flag, then use a thread to render)
    }
}
