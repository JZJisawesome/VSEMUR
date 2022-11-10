/* logging.rs
 * By: John Jekel
 *
 * Logging facilities for libvsemur
 *
*/

/* Imports */

//TODO (include "use" and "mod" here)

/* Constants */

//TODO

/* Macros */

//Thanks https://stackoverflow.com/questions/34373169/how-do-i-create-a-rust-macro-with-optional-parameters-using-repetitions
macro_rules! log_noln {
    //Case where there are no extra arguments
    ($tick_num:expr, $indent:expr, $string:expr) => {
        if cfg!(debug_assertions) {
            eprint!("\x1b[32m@t=\x1b[95m{:>10}\x1b[34m>\x1b[0m ", $tick_num);
            for _ in 0..$indent {
                eprint!("  ");
            }
            eprint!($string);
        }
    };
    //Case where there are extra arguments
    /*//FIXME this dosn't work (how to pass multiple args to eprintln???)
    ($tick_num:expr, $indent:expr, $string:expr, $($extra_println_args:expr), +) => {
        if cfg!(debug_assertions) {
            eprint!("\x1b[32m@t=\x1b[95m{:>10}\x1b[34m>\x1b[0m ", $tick_num);
            for _ in 0..$indent {
                eprint!("  ");
            }
            //TODO get this to work (may need to example $string ourselves...)
            eprintln!($string, $extra_println_args);
        }
    };
    */
    //HACK Support up to three extra arguments (the most we'll likely need; add more if necessary)
    ($tick_num:expr, $indent:expr, $string:expr, $extra_arg_1:expr) => {
        if cfg!(debug_assertions) {
            eprint!("\x1b[32m@t=\x1b[95m{:>10}\x1b[34m>\x1b[0m ", $tick_num);
            for _ in 0..$indent {
                eprint!("  ");
            }
            eprint!($string, $extra_arg_1);
        }
    };
    ($tick_num:expr, $indent:expr, $string:expr, $extra_arg_1:expr, $extra_arg_2:expr) => {
        if cfg!(debug_assertions) {
            eprint!("\x1b[32m@t=\x1b[95m{:>10}\x1b[34m>\x1b[0m ", $tick_num);
            for _ in 0..$indent {
                eprint!("  ");
            }
            eprint!($string, $extra_arg_1, $extra_arg_2);
        }
    };
    ($tick_num:expr, $indent:expr, $string:expr, $extra_arg_1:expr, $extra_arg_2:expr, $extra_arg_3:expr) => {
        if cfg!(debug_assertions) {
            eprint!("\x1b[32m@t=\x1b[95m{:>10}\x1b[34m>\x1b[0m ", $tick_num);
            for _ in 0..$indent {
                eprint!("  ");
            }
            eprint!($string, $extra_arg_1, $extra_arg_2, $extra_arg_3);
        }
    };
}
pub(crate) use log_noln;

macro_rules! log_finln {
    //Case where there are no arguments at all
    () => {
        if cfg!(debug_assertions) {
            eprint!("\n");
        }
    };

    //Case where there are no extra arguments
    ($string:expr) => {
        if cfg!(debug_assertions) {
            eprintln!($string);
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
            eprintln!($string, $extra_arg_1);
        }
    };
    ($string:expr, $extra_arg_1:expr, $extra_arg_2:expr) => {
        if cfg!(debug_assertions) {
            eprintln!($string, $extra_arg_1, $extra_arg_2);
        }
    };
    ($string:expr, $extra_arg_1:expr, $extra_arg_2:expr, $extra_arg_3:expr) => {
        if cfg!(debug_assertions) {
            eprintln!($string, $extra_arg_1, $extra_arg_2, $extra_arg_3);
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

/* Static Variables */

//TODO

/* Types */

//TODO

/* Associated Functions and Methods */

//TODO

/* Functions */

//TODO
