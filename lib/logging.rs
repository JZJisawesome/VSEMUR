
//Thanks https://stackoverflow.com/questions/34373169/how-do-i-create-a-rust-macro-with-optional-parameters-using-repetitions
macro_rules! log {
    //Case where there are no extra arguments
    ($tick_num:expr, $indent:expr, $string:expr) => {
        if cfg!(debug_assertions) {
            eprint!("\x1b[32m@Tick=\x1b[95m{}\x1b[34m>\x1b[0m\t", $tick_num);
            for _ in 0..$indent {
                eprint!("\t");
            }
            eprintln!($string);
        }
    };
    //Case where there are extra arguments
    /*//FIXME this dosn't work (how to pass multiple args to eprintln???)
    ($tick_num:expr, $indent:expr, $string:expr, $($extra_println_args:expr), +) => {
        if cfg!(debug_assertions) {
            eprint!("\x1b[32m@Tick=\x1b[95m{}\x1b[34m>\x1b[0m\t", $tick_num);
            for _ in 0..$indent {
                eprint!("\t");
            }
            //TODO get this to work (may need to example $string ourselves...)
            eprintln!($string, $extra_println_args);
        }
    };
    */
    //HACK Support up to three extra arguments (the most we'll likely need; add more if necessary)
    ($tick_num:expr, $indent:expr, $string:expr, $extra_arg_1:expr) => {
        if cfg!(debug_assertions) {
            eprint!("\x1b[32m@Tick=\x1b[95m{}\x1b[34m>\x1b[0m\t", $tick_num);
            for _ in 0..$indent {
                eprint!("\t");
            }
            eprintln!($string, $extra_arg_1);
        }
    };
    ($tick_num:expr, $indent:expr, $string:expr, $extra_arg_1:expr, $extra_arg_2:expr) => {
        if cfg!(debug_assertions) {
            eprint!("\x1b[32m@Tick=\x1b[95m{}\x1b[34m>\x1b[0m\t", $tick_num);
            for _ in 0..$indent {
                eprint!("\t");
            }
            eprintln!($string, $extra_arg_1, $extra_arg_2);
        }
    };
    ($tick_num:expr, $indent:expr, $string:expr, $extra_arg_1:expr, $extra_arg_2:expr, $extra_arg_3:expr) => {
        if cfg!(debug_assertions) {
            eprint!("\x1b[32m@Tick=\x1b[95m{}\x1b[34m>\x1b[0m\t", $tick_num);
            for _ in 0..$indent {
                eprint!("\t");
            }
            eprintln!($string, $extra_arg_1, $extra_arg_2, $extra_arg_3);
        }
    };
}
pub(crate) use log;
