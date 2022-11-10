/* interpreter.rs: VSEMUR Interpreter
 * By: John Jekel
 *
 * Emulates a VSmile system one .tick() at a time!
 *
*/
mod execute;
mod memory;


use crate::logging::log;


pub struct State {
    t: u128,
    regs: Registers,
    buttons: Buttons,
    //TODO how to allocate memory in rust w/o pointers?
}

pub enum ReturnCode {OK, FAIL, EXIT_NORMAL}//TODO error codes/success


const MEMORY_SIZE_BYTES: usize = std::mem::size_of::<u16>() * (1 << 22);


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

//Only functions called by external users are associated functions/methods
//Everything else goes into other modules and are not associated
impl State {
    pub fn new() -> State {
        log!(0, 0, "Initialized VSEMUR State");

        return State {
            t: 0,
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

    pub fn reset(self: &mut Self) -> ReturnCode {
        //unimplemented!();//TODO implement
        return ReturnCode::FAIL;
    }

    pub fn tick(self: &mut Self) -> ReturnCode {
        self.t += 1;
        log!(self.t, 0, "Tick {} begins", self.t);
        memory::fetch(&self);
        return ReturnCode::FAIL;
    }

    pub fn load_bios_file(self: &mut Self, path: &str) -> ReturnCode {
        //unimplemented!();//TODO implement
        //Merely copies the bios into a seperate buffer field in memory; does not load it into the emulated system's memory (that is done on reset)
        return ReturnCode::FAIL;
    }

    pub fn load_bios_mem(self: &mut Self/* TODO take pointer to memory or something similar*/) -> ReturnCode {
        //unimplemented!();//TODO implement
        //Merely copies the bios into a seperate buffer field in memory; does not load it into the emulated system's memory (that is done on reset)
        return ReturnCode::FAIL;
    }

    pub fn load_rom_file(self: &mut Self, path: &str) -> ReturnCode {
        //unimplemented!();//TODO implement
        //Merely copies the bios into a seperate buffer field in memory; does not load it into the emulated system's memory (that is done on reset)
        return ReturnCode::FAIL;
    }

    pub fn load_rom_mem(self: &mut Self/* TODO take pointer to memory or something similar*/) -> ReturnCode {
        //unimplemented!();//TODO implement
        //Merely copies the bios into a seperate buffer field in memory; does not load it into the emulated system's memory (that is done on reset)
        return ReturnCode::FAIL;
    }
}

struct Inst {
    pub(crate) wg: [u16; 2],
}
