
//TODO other functions
//TODO support ZSTD-compressed roms

//Thanks https://stackoverflow.com/questions/67307526/is-it-possible-to-get-the-cargo-environment-variable-information-for-one-crate-u
//pub use vsemur::version;
pub mod about;
mod lifetime;
mod logging;



pub struct State {
    num_ticks: u128,
    regs: Registers,
    buttons: Buttons,
    //TODO how to allocate memory in rust w/o pointers?
}

pub enum ReturnCode {OK, FAIL}//TODO error codes/success

pub fn tick(state: &mut State) -> ReturnCode {
    state.num_ticks += 1;
    logging::log!(state.num_ticks, 0, "Tick {} begins", state.num_ticks);
    //TODO
    return ReturnCode::FAIL;
}




struct ControllerButtons {
    red: bool,
    yellow: bool,
    blue: bool,
    green: bool,

    enter: bool,
    help: bool,
    exit: bool,
    abc: bool,

    up: bool,
    down: bool,
    left: bool,
    right: bool,

    //TODO figure out how to handle the touchpads if they exist
}

struct Buttons {
    p1: ControllerButtons,
    p2: ControllerButtons,
}

struct SR {
    ds: u8,
    cs: u8,

    n: bool,
    z: bool,
    s: bool,
    c: bool,
}

struct Registers {
    sp: u32,//Only need 22 bits
    r: [u16;4],
    bp: u16,
    sr: SR,
    pc: u32,//Only need 22 bits
}

impl State {
    pub fn new(/*todo args*/) -> State {
        logging::log!(0, 0, "Initialized VSEMUR State");

        return State {
            num_ticks: 0,
            regs: Registers {
                sp: 0,//TODO this will be different
                r: [0, 0, 0, 0],
                bp: 0,
                sr: SR {
                    ds: 0,
                    cs: 0,

                    n: false,
                    z: false,
                    s: false,
                    c: false
                },
                pc: 0,//TODO this will be different
            },
            buttons: Buttons {
                p1: ControllerButtons {
                    red: false,
                    yellow: false,
                    blue: false,
                    green: false,

                    enter: false,
                    help: false,
                    exit: false,
                    abc: false,

                    up: false,
                    down: false,
                    left: false,
                    right: false,
                },
                p2: ControllerButtons {
                    red: false,
                    yellow: false,
                    blue: false,
                    green: false,

                    enter: false,
                    help: false,
                    exit: false,
                    abc: false,

                    up: false,
                    down: false,
                    left: false,
                    right: false,
                },
            },
            //TODO other fields
        };
    }
}
