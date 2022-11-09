
//Thanks https://stackoverflow.com/questions/34373169/how-do-i-create-a-rust-macro-with-optional-parameters-using-repetitions

macro_rules! log {
    ($tick_num:expr, $indent:expr, $string:expr) => {
        if cfg!(debug_assertions) {
            //TODO indent
            print!("\x1b[32m@Tick=\x1b[95m{}\x1b[34m>\x1b[0m\t", $tick_num);//TODO add param for tick
            println!($string);
        }
    };
    ($tick_num:expr, $indent:expr, $string:expr, $($others:expr), *) => {
        if cfg!(debug_assertions) {
            print!("test:");
            //TODO get this to work
            //println!($string, $others);
        }
    };
}

pub(crate) use log;//TODO avoid this being visible outside of the crate


fn log_func() {
    if cfg!(debug_assertions) {

    }
}
