/* logging.rs
 * Copyright (C) 2022 John Jekel
 * See the LICENSE file at the root of the project for licensing info.
 *
 * Logging facilities for libvsemur (debug builds only)
 * These do nothing in release builds (optimized away by the compiler)
 *
*/

//We don't want warnings if the loggings statements are unreachable when used elsewhere
#![allow(unreachable_code)]

/* Imports */

/* Constants */

pub(crate) const LOG_FILE_PATH: &str = "/tmp/vsemur-log.txt";

/* Static Variables */

//Is this thread safe? No.
//Is this a variable accessible to the whole crate? Yes.
//Is this unsafe rust? Yes.
//But it is only used in debug builds, and saves us having to pass a cycle block variable to each function that needs to log something
//thus improving the performance of release builds
pub(crate) static mut CYCLE_BLOCK_NUM: u128 = 0;

/* Macros */

macro_rules! internal_log_prompt {//Helper macro
    ($indent:expr) => {
        eprint!("\x1b[32m@cb=\x1b[95m{:>9}\x1b[1;34m>\x1b[0m ", crate::logging::internal_log_ticks!());
        for _ in 0..$indent {
            eprint!("  ");
        }
    };
}
pub(crate) use internal_log_prompt;
macro_rules! internal_log_file_open {//Helper macro
    () => {
        std::fs::OpenOptions::new().append(true).write(true).create(true).open(crate::logging::LOG_FILE_PATH).unwrap()
    };
}
pub(crate) use internal_log_file_open;
macro_rules! internal_log_buffer_create {//Helper macro
    ($log_file:expr) => {
        std::io::BufWriter::new($log_file)
    };
}
pub(crate) use internal_log_buffer_create;
macro_rules! internal_log_buffer_create_and_prompt {//Helper macro
    ($log_file:expr, $indent:expr) => {{
        let mut log_buffer = crate::logging::internal_log_buffer_create!($log_file);
        use std::io::Write;
        write!(&mut log_buffer, "@cb={:>9}> ", crate::logging::internal_log_ticks!()).unwrap();
        for _ in 0..$indent {
            write!(&mut log_buffer, "  ").unwrap();
        }
        log_buffer
    }};
}
pub(crate) use internal_log_buffer_create_and_prompt;
macro_rules! internal_log_ticks {//Helper macro
    () => {
        unsafe { crate::logging::CYCLE_BLOCK_NUM }
    };
}
pub(crate) use internal_log_ticks;

macro_rules! log_reset_ticks {
    () => {
        if cfg!(debug_assertions) && !cfg!(test) {
            unsafe { crate::logging::CYCLE_BLOCK_NUM = 0; }
        }
    };
}
pub(crate) use log_reset_ticks;
macro_rules! log_increment_ticks {
    () => {
        if cfg!(debug_assertions) && !cfg!(test) {
            unsafe { crate::logging::CYCLE_BLOCK_NUM += 1; }
        }
    };
}
pub(crate) use log_increment_ticks;
//Thanks https://stackoverflow.com/questions/34373169/how-do-i-create-a-rust-macro-with-optional-parameters-using-repetitions
macro_rules! log_noln {
    //Case where there are no extra arguments
    ($indent:expr, $string:expr) => {
        if cfg!(debug_assertions) && !cfg!(test) {
            //Log to stderr
            crate::logging::internal_log_prompt!($indent);
            eprint!($string);

            //Log to the log file
            use std::io::Write;
            let log_file = crate::logging::internal_log_file_open!();
            let mut log_buffer = crate::logging::internal_log_buffer_create_and_prompt!(&log_file, $indent);
            write!(&mut log_buffer, $string).unwrap();
        }
    };
    //Case where there are extra arguments
    /*//FIXME this dosn't work (how to pass multiple args to eprintln???)
    ($indent:expr, $string:expr, $($extra_println_args:expr), +) => {

    };
    */
    //HACK Support up to three extra arguments (the most we'll likely need; add more if necessary)
    ($indent:expr, $string:expr, $extra_arg_1:expr) => {
        if cfg!(debug_assertions) && !cfg!(test) {
            //Log to stderr
            crate::logging::internal_log_prompt!($indent);
            eprint!($string, $extra_arg_1);

            //Log to the log file
            use std::io::Write;
            let log_file = crate::logging::internal_log_file_open!();
            let mut log_buffer = crate::logging::internal_log_buffer_create_and_prompt!(&log_file, $indent);
            write!(&mut log_buffer, $string, $extra_arg_1).unwrap();
        }
    };
    ($indent:expr, $string:expr, $extra_arg_1:expr, $extra_arg_2:expr) => {
        if cfg!(debug_assertions) && !cfg!(test) {
            //Log to stderr
            crate::logging::internal_log_prompt!($indent);
            eprint!($string, $extra_arg_1, $extra_arg_2);

            //Log to the log file
            use std::io::Write;
            let log_file = crate::logging::internal_log_file_open!();
            let mut log_buffer = crate::logging::internal_log_buffer_create_and_prompt!(&log_file, $indent);
            write!(&mut log_buffer, $string, $extra_arg_1, $extra_arg_2).unwrap();
        }
    };
    ($indent:expr, $string:expr, $extra_arg_1:expr, $extra_arg_2:expr, $extra_arg_3:expr) => {
        if cfg!(debug_assertions) && !cfg!(test) {
            //Log to stderr
            crate::logging::internal_log_prompt!($indent);
            eprint!($string, $extra_arg_1, $extra_arg_2, $extra_arg_3);

            //Log to the log file
            use std::io::Write;
            let log_file = crate::logging::internal_log_file_open!();
            let mut log_buffer = crate::logging::internal_log_buffer_create_and_prompt!(&log_file, $indent);
            write!(&mut log_buffer, $string, $extra_arg_1, $extra_arg_2, $extra_arg_3).unwrap();
        }
    };
}
pub(crate) use log_noln;

macro_rules! log_midln {
    //Case where there are no extra arguments
    ($string:expr) => {
        if cfg!(debug_assertions) && !cfg!(test) {
            //Log to stderr
            eprint!($string);

            //Log to the log file
            use std::io::Write;
            let log_file = crate::logging::internal_log_file_open!();
            let mut log_buffer = crate::logging::internal_log_buffer_create!(&log_file);
            write!(&mut log_buffer, $string).unwrap();
        }
    };
    //Case where there are extra arguments
    /*//FIXME this dosn't work (how to pass multiple args to log_noln???)
    ($string:expr, $($extra_println_args:expr), +) => {

    };
    */
    //HACK Support up to three extra arguments (the most we'll likely need; add more if necessary)
    ($string:expr, $extra_arg_1:expr) => {
        if cfg!(debug_assertions) && !cfg!(test) {
            //Log to stderr
            eprint!($string, $extra_arg_1);

            //Log to the log file
            use std::io::Write;
            let log_file = crate::logging::internal_log_file_open!();
            let mut log_buffer = crate::logging::internal_log_buffer_create!(&log_file);
            write!(&mut log_buffer, $string, $extra_arg_1, $extra_arg_2).unwrap();
        }
    };
    ($string:expr, $extra_arg_1:expr, $extra_arg_2:expr) => {
        if cfg!(debug_assertions) && !cfg!(test) {
            //Log to stderr
            eprint!($string, $extra_arg_1, $extra_arg_2);

            //Log to the log file
            use std::io::Write;
            let log_file = crate::logging::internal_log_file_open!();
            let mut log_buffer = crate::logging::internal_log_buffer_create!(&log_file);
            write!(&mut log_buffer, $string).unwrap();
        }
    };
    ($string:expr, $extra_arg_1:expr, $extra_arg_2:expr, $extra_arg_3:expr) => {
        if cfg!(debug_assertions) && !cfg!(test) {
            //Log to stderr
            eprint!($string, $extra_arg_1, $extra_arg_2, $extra_arg_3);

            //Log to the log file
            use std::io::Write;
            let log_file = crate::logging::internal_log_file_open!();
            let mut log_buffer = crate::logging::internal_log_buffer_create!(&log_file);
            write!(&mut log_buffer, $string, $extra_arg_1, $extra_arg_2, $extra_arg_3).unwrap();
        }
    };
}
pub(crate) use log_midln;

macro_rules! log_finln {
    //Case where there are no arguments at all
    () => {
        if cfg!(debug_assertions) && !cfg!(test) {
            //Log to stderr
            eprint!("\n");

            //Log to the log file
            use std::io::Write;
            let log_file = crate::logging::internal_log_file_open!();
            let mut log_buffer = crate::logging::internal_log_buffer_create!(&log_file);
            write!(&mut log_buffer, "\n").unwrap();
        }
    };

    //Case where there are no extra arguments
    ($string:expr) => {
        if cfg!(debug_assertions) && !cfg!(test) {
            //Log to stderr
            eprintln!($string);

            //Log to the log file
            use std::io::Write;
            let log_file = crate::logging::internal_log_file_open!();
            let mut log_buffer = crate::logging::internal_log_buffer_create!(&log_file);
            writeln!(&mut log_buffer, $string).unwrap();
        }
    };
    //Case where there are extra arguments
    /*//FIXME this dosn't work (how to pass multiple args to log_noln???)
    ($string:expr, $($extra_println_args:expr), +) => {

    };
    */
    //HACK Support up to three extra arguments (the most we'll likely need; add more if necessary)
    ($string:expr, $extra_arg_1:expr) => {
        if cfg!(debug_assertions) && !cfg!(test) {
            //Log to stderr
            eprintln!($string, $extra_arg_1);

            //Log to the log file
            use std::io::Write;
            let log_file = crate::logging::internal_log_file_open!();
            let mut log_buffer = crate::logging::internal_log_buffer_create!(&log_file);
            writeln!(&mut log_buffer, $string, $extra_arg_1).unwrap();
        }
    };
    ($string:expr, $extra_arg_1:expr, $extra_arg_2:expr) => {
        if cfg!(debug_assertions) && !cfg!(test) {
            //Log to stderr
            eprintln!($string, $extra_arg_1, $extra_arg_2);

            //Log to the log file
            use std::io::Write;
            let log_file = crate::logging::internal_log_file_open!();
            let mut log_buffer = crate::logging::internal_log_buffer_create!(&log_file);
            writeln!(&mut log_buffer, $string, $extra_arg_1, $extra_arg_2).unwrap();
        }
    };
    ($string:expr, $extra_arg_1:expr, $extra_arg_2:expr, $extra_arg_3:expr) => {
        if cfg!(debug_assertions) && !cfg!(test) {
            //Log to stderr
            eprintln!($string, $extra_arg_1, $extra_arg_2, $extra_arg_3);

            //Log to the log file
            use std::io::Write;
            let log_file = crate::logging::internal_log_file_open!();
            let mut log_buffer = crate::logging::internal_log_buffer_create!(&log_file);
            writeln!(&mut log_buffer, $string, $extra_arg_1, $extra_arg_2, $extra_arg_3).unwrap();
        }
    };
}
pub(crate) use log_finln;

macro_rules! log {
    //Case where there are no extra arguments
    ($indent:expr, $string:expr) => {
        if cfg!(debug_assertions) && !cfg!(test) {
            crate::logging::log_noln!($indent, $string);
            crate::logging::log_finln!();
        }
    };
    //Case where there are extra arguments
    /*//FIXME this dosn't work (how to pass multiple args to log_noln???)
    ($indent:expr, $string:expr, $($extra_println_args:expr), +) => {

    };
    */
    //HACK Support up to three extra arguments (the most we'll likely need; add more if necessary)
    ($indent:expr, $string:expr, $extra_arg_1:expr) => {
        if cfg!(debug_assertions) && !cfg!(test) {
            crate::logging::log_noln!($indent, $string, $extra_arg_1);
            crate::logging::log_finln!();
        }
    };
    ($indent:expr, $string:expr, $extra_arg_1:expr, $extra_arg_2:expr) => {
        if cfg!(debug_assertions) && !cfg!(test) {
            crate::logging::log_noln!($indent, $string, $extra_arg_1, $extra_arg_2);
            crate::logging::log_finln!();
        }
    };
    ($indent:expr, $string:expr, $extra_arg_1:expr, $extra_arg_2:expr, $extra_arg_3:expr) => {
        if cfg!(debug_assertions) && !cfg!(test) {
            crate::logging::log_noln!($indent, $string, $extra_arg_1, $extra_arg_2, $extra_arg_3);
            crate::logging::log_finln!();
        }
    };
}
pub(crate) use log;

macro_rules! log_ansi {
    //Case where there are no extra arguments
    ($indent:expr, $ansi:expr, $string:expr) => {
        if cfg!(debug_assertions) && !cfg!(test) {
            crate::logging::log_noln!($indent, "");
            eprint!($ansi);//Only output ansi to the terminal, not to the log file
            crate::logging::log_finln!($string);
            eprint!("\x1b[0m");//Reset ansi text properties
        }
    };
    //Case where there are extra arguments
    /*//FIXME this dosn't work (how to pass multiple args to log_noln???)
    ($indent:expr, $string:expr, $($extra_println_args:expr), +) => {

    };
    */
    //HACK Support up to three extra arguments (the most we'll likely need; add more if necessary)
    ($indent:expr, $ansi:expr, $string:expr, $extra_arg_1:expr) => {
        if cfg!(debug_assertions) && !cfg!(test) {
            crate::logging::log_noln!($indent, "");
            eprint!($ansi);//Only output ansi to the terminal, not to the log file
            crate::logging::log_finln!($string, $extra_arg_1);
            eprint!("\x1b[0m");//Reset ansi text properties
        }
    };
    ($indent:expr, $ansi:expr, $string:expr, $extra_arg_1:expr, $extra_arg_2:expr) => {
        if cfg!(debug_assertions) && !cfg!(test) {
            crate::logging::log_noln!($indent, "");
            eprint!($ansi);//Only output ansi to the terminal, not to the log file
            crate::logging::log_finln!($string, $extra_arg_1, $extra_arg_2);
            eprint!("\x1b[0m");//Reset ansi text properties
        }
    };
    ($indent:expr, $ansi:expr, $string:expr, $extra_arg_1:expr, $extra_arg_2:expr, $extra_arg_3:expr) => {
        if cfg!(debug_assertions) && !cfg!(test) {
            crate::logging::log_noln!($indent, "");
            eprint!($ansi);//Only output ansi to the terminal, not to the log file
            crate::logging::log_finln!($string, $extra_arg_1, $extra_arg_2, $extra_arg_3);
            eprint!("\x1b[0m");//Reset ansi text properties
        }
    };
}
pub(crate) use log_ansi;

macro_rules! log_reset_file {
    () => {
        if cfg!(debug_assertions) && !cfg!(test) {
            match std::fs::remove_file(crate::logging::LOG_FILE_PATH) {
                Ok(_) => {
                    crate::logging::log_ansi!(0, "\x1b[36m", "Overwriting existing log file \"{}\"", crate::logging::LOG_FILE_PATH);
                },
                Err(_) => {
                    crate::logging::log_ansi!(0, "\x1b[36m", "Creating new log file \"{}\"", crate::logging::LOG_FILE_PATH);
                }
            }
        }
    }
}
pub(crate) use log_reset_file;

/* Types */

//TODO

/* Associated Functions and Methods */

//TODO

/* Functions */

//TODO
