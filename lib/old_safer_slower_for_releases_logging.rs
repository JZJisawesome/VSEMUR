/* logging.rs
 * By: John Jekel
 *
 * Logging facilities for libvsemur
 *
*/

//We don't want warnings if the loggings statements are unreachable when used elsewhere
#![allow(unreachable_code)]

/* Imports */

/* Constants */

pub(crate) const LOG_FILE_PATH: &str = "vsemur-log.txt";

/* Macros */

//FIXME prevent having to export these helper macros to the whole crate (limitation of rust)
macro_rules! internal_log_prompt {//Helper macro
    ($tick_num:expr, $indent:expr) => {
        eprint!("\x1b[32m@t=\x1b[95m{:>10}\x1b[1;34m>\x1b[0m ", $tick_num);
        for _ in 0..$indent {
            eprint!("  ");
        }
    };
}
pub(crate) use internal_log_prompt;//FIXME prevent having to export these helper macros to the whole crate (limitation of rust)
macro_rules! internal_log_file_open {//Helper macro
    () => {
        std::fs::OpenOptions::new().append(true).write(true).create(true).open(crate::logging::LOG_FILE_PATH).unwrap()
    };
}
pub(crate) use internal_log_file_open;//FIXME prevent having to export these helper macros to the whole crate (limitation of rust)
macro_rules! internal_log_buffer_create {//Helper macro
    ($log_file:expr) => {
        std::io::BufWriter::new($log_file)
    };
}
pub(crate) use internal_log_buffer_create;//FIXME prevent having to export these helper macros to the whole crate (limitation of rust)
macro_rules! internal_log_buffer_create_and_prompt {//Helper macro
    ($log_file:expr, $tick_num:expr, $indent:expr) => {{
        let mut log_buffer = crate::logging::internal_log_buffer_create!($log_file);
        use std::io::Write;
        write!(&mut log_buffer, "@t={:>10}> ", $tick_num).unwrap();
        for _ in 0..$indent {
            write!(&mut log_buffer, "  ").unwrap();
        }
        log_buffer
    }};
}
pub(crate) use internal_log_buffer_create_and_prompt;//FIXME prevent having to export these helper macros to the whole crate (limitation of rust)

//Thanks https://stackoverflow.com/questions/34373169/how-do-i-create-a-rust-macro-with-optional-parameters-using-repetitions
macro_rules! log_noln {
    //Case where there are no extra arguments
    ($tick_num:expr, $indent:expr, $string:expr) => {
        if cfg!(debug_assertions) {
            //Log to stderr
            crate::logging::internal_log_prompt!($tick_num, $indent);
            eprint!($string);

            //Log to the log file
            use std::io::Write;
            let log_file = crate::logging::internal_log_file_open!();
            let mut log_buffer = crate::logging::internal_log_buffer_create_and_prompt!(&log_file, $tick_num, $indent);
            write!(&mut log_buffer, $string).unwrap();
        }
    };
    //Case where there are extra arguments
    /*//FIXME this dosn't work (how to pass multiple args to eprintln???)
    ($tick_num:expr, $indent:expr, $string:expr, $($extra_println_args:expr), +) => {

    };
    */
    //HACK Support up to three extra arguments (the most we'll likely need; add more if necessary)
    ($tick_num:expr, $indent:expr, $string:expr, $extra_arg_1:expr) => {
        if cfg!(debug_assertions) {
            //Log to stderr
            crate::logging::internal_log_prompt!($tick_num, $indent);
            eprint!($string, $extra_arg_1);

            //Log to the log file
            use std::io::Write;
            let log_file = crate::logging::internal_log_file_open!();
            let mut log_buffer = crate::logging::internal_log_buffer_create_and_prompt!(&log_file, $tick_num, $indent);
            write!(&mut log_buffer, $string, $extra_arg_1).unwrap();
        }
    };
    ($tick_num:expr, $indent:expr, $string:expr, $extra_arg_1:expr, $extra_arg_2:expr) => {
        if cfg!(debug_assertions) {
            //Log to stderr
            crate::logging::internal_log_prompt!($tick_num, $indent);
            eprint!($string, $extra_arg_1, $extra_arg_2);

            //Log to the log file
            use std::io::Write;
            let log_file = crate::logging::internal_log_file_open!();
            let mut log_buffer = crate::logging::internal_log_buffer_create_and_prompt!(&log_file, $tick_num, $indent);
            write!(&mut log_buffer, $string, $extra_arg_1, $extra_arg_2).unwrap();
        }
    };
    ($tick_num:expr, $indent:expr, $string:expr, $extra_arg_1:expr, $extra_arg_2:expr, $extra_arg_3:expr) => {
        if cfg!(debug_assertions) {
            //Log to stderr
            crate::logging::internal_log_prompt!($tick_num, $indent);
            eprint!($string, $extra_arg_1, $extra_arg_2, $extra_arg_3);

            //Log to the log file
            use std::io::Write;
            let log_file = crate::logging::internal_log_file_open!();
            let mut log_buffer = crate::logging::internal_log_buffer_create_and_prompt!(&log_file, $tick_num, $indent);
            write!(&mut log_buffer, $string, $extra_arg_1, $extra_arg_2, $extra_arg_3).unwrap();
        }
    };
}
pub(crate) use log_noln;

macro_rules! log_midln {
    //Case where there are no extra arguments
    ($string:expr) => {
        if cfg!(debug_assertions) {
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
        if cfg!(debug_assertions) {
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
        if cfg!(debug_assertions) {
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
        if cfg!(debug_assertions) {
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
        if cfg!(debug_assertions) {
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
        if cfg!(debug_assertions) {
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
        if cfg!(debug_assertions) {
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
        if cfg!(debug_assertions) {
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
        if cfg!(debug_assertions) {
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
    ($tick_num:expr, $indent:expr, $string:expr) => {
        if cfg!(debug_assertions) {
            crate::logging::log_noln!($tick_num, $indent, $string);
            crate::logging::log_finln!();
        }
    };
    //Case where there are extra arguments
    /*//FIXME this dosn't work (how to pass multiple args to log_noln???)
    ($tick_num:expr, $indent:expr, $string:expr, $($extra_println_args:expr), +) => {

    };
    */
    //HACK Support up to three extra arguments (the most we'll likely need; add more if necessary)
    ($tick_num:expr, $indent:expr, $string:expr, $extra_arg_1:expr) => {
        if cfg!(debug_assertions) {
            crate::logging::log_noln!($tick_num, $indent, $string, $extra_arg_1);
            crate::logging::log_finln!();
        }
    };
    ($tick_num:expr, $indent:expr, $string:expr, $extra_arg_1:expr, $extra_arg_2:expr) => {
        if cfg!(debug_assertions) {
            crate::logging::log_noln!($tick_num, $indent, $string, $extra_arg_1, $extra_arg_2);
            crate::logging::log_finln!();
        }
    };
    ($tick_num:expr, $indent:expr, $string:expr, $extra_arg_1:expr, $extra_arg_2:expr, $extra_arg_3:expr) => {
        if cfg!(debug_assertions) {
            crate::logging::log_noln!($tick_num, $indent, $string, $extra_arg_1, $extra_arg_2, $extra_arg_3);
            crate::logging::log_finln!();
        }
    };
}
pub(crate) use log;

macro_rules! log_ansi {
    //Case where there are no extra arguments
    ($tick_num:expr, $indent:expr, $ansi:expr, $string:expr) => {
        if cfg!(debug_assertions) {
            crate::logging::log_noln!($tick_num, $indent, "");
            eprint!($ansi);//Only output ansi to the terminal, not to the log file
            crate::logging::log_finln!($string);
            eprint!("\x1b[0m");//Reset ansi text properties
        }
    };
    //Case where there are extra arguments
    /*//FIXME this dosn't work (how to pass multiple args to log_noln???)
    ($tick_num:expr, $indent:expr, $string:expr, $($extra_println_args:expr), +) => {

    };
    */
    //HACK Support up to three extra arguments (the most we'll likely need; add more if necessary)
    ($tick_num:expr, $indent:expr, $ansi:expr, $string:expr, $extra_arg_1:expr) => {
        if cfg!(debug_assertions) {
            crate::logging::log_noln!($tick_num, $indent, "");
            eprint!($ansi);//Only output ansi to the terminal, not to the log file
            crate::logging::log_finln!($string, $extra_arg_1);
            eprint!("\x1b[0m");//Reset ansi text properties
        }
    };
    ($tick_num:expr, $indent:expr, $ansi:expr, $string:expr, $extra_arg_1:expr, $extra_arg_2:expr) => {
        if cfg!(debug_assertions) {
            crate::logging::log_noln!($tick_num, $indent, "");
            eprint!($ansi);//Only output ansi to the terminal, not to the log file
            crate::logging::log_finln!($string, $extra_arg_1, $extra_arg_2);
            eprint!("\x1b[0m");//Reset ansi text properties
        }
    };
    ($tick_num:expr, $indent:expr, $ansi:expr, $string:expr, $extra_arg_1:expr, $extra_arg_2:expr, $extra_arg_3:expr) => {
        if cfg!(debug_assertions) {
            crate::logging::log_noln!($tick_num, $indent, "");
            eprint!($ansi);//Only output ansi to the terminal, not to the log file
            crate::logging::log_finln!($string, $extra_arg_1, $extra_arg_2, $extra_arg_3);
            eprint!("\x1b[0m");//Reset ansi text properties
        }
    };
}
pub(crate) use log_ansi;

macro_rules! log_reset_file {
    () => {
        if cfg!(debug_assertions) {
            match std::fs::remove_file(crate::logging::LOG_FILE_PATH) {
                Ok(_) => {
                    crate::logging::log_ansi!(0, 0, "\x1b[36m", "Overwriting existing log file \"{}\"", crate::logging::LOG_FILE_PATH);
                },
                Err(_) => {
                    crate::logging::log_ansi!(0, 0, "\x1b[36m", "Creating new log file \"{}\"", crate::logging::LOG_FILE_PATH);
                }
            }
        }
    }
}
pub(crate) use log_reset_file;

/* Static Variables */

//TODO

/* Types */

//TODO

/* Associated Functions and Methods */

//TODO

/* Functions */

//TODO
