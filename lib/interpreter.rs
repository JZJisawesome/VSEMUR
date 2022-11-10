/* interpreter.rs: VSEMUR Interpreter
 * By: John Jekel
 *
 * Emulates a VSmile system one .tick() at a time!
 *
*/

/* Imports */

mod execute;
mod memory;

use crate::logging::log;

use std::fs::File;
use std::io::Read;

/* Constants */

const MAX_BIOS_SIZE_BYTES: usize = std::mem::size_of::<u16>() * (1 << 22);
const MAX_ROM_SIZE_BYTES: usize = std::mem::size_of::<u16>() * (1 << 22);
const MEM_SIZE_BYTES: usize = std::mem::size_of::<u16>() * (1 << 22);

/* Macros */

//TODO (also pub(crate) use the_macro statements here too)

/* Static Variables */

//TODO

/* Types */

pub struct State {
    t: u128,
    regs: Registers,
    buttons: Buttons,
    bios_loaded: bool,
    rom_loaded: bool,
    mem_loaded: bool,
    bios: Box<[u8]>,
    rom: Box<[u8]>,
    mem: Box<[u8]>,
    //TODO how to allocate memory in rust w/o pointers?
}

pub enum ReturnCode {
    TICK_OK,
    TICK_FAIL,
    TICK_FAIL_FETCH,
    TICK_FAIL_EXECUTE,
    TICK_EXIT_NORMAL,

    RESET_OK,
    RESET_FAIL,

    LOAD_OK,
    LOAD_FAIL_OPEN,
    LOAD_FAIL_SIZE,

}//TODO error codes/success


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
    sp: u16,
    r: [u16;4],
    bp: u16,
    sr: SR,
    pc: u16,
}

struct Inst {
    pub(crate) wg: [u16; 2],
}

/* Associated Functions and Methods */

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
            /*
            //FIXME use this instead once it is stable
            bios: box [0; MAX_BIOS_SIZE_BYTES],//TODO avoid zero-initializing for speed
            rom: box [0; MAX_ROM_SIZE_BYTES],//TODO avoid zero-initializing for speed
            mem: box [0; MEM_SIZE_BYTES],//TODO avoid zero-initializing for speed
            */
            bios_loaded: false,
            rom_loaded: false,
            mem_loaded: false,
            bios: vec![0u8; MAX_BIOS_SIZE_BYTES].into_boxed_slice(),//TODO avoid vector for speed//TODO avoid zero-initializing for speed
            rom: vec![0u8; MAX_ROM_SIZE_BYTES].into_boxed_slice(),//TODO avoid vector for speed//TODO avoid zero-initializing for speed
            mem: vec![0u8; MEM_SIZE_BYTES].into_boxed_slice(),//TODO avoid vector for speed//TODO avoid zero-initializing for speed
            //TODO other fields
        };
    }

    pub fn reset(self: &mut Self) -> ReturnCode {
        if !self.bios_loaded || !self.rom_loaded {
            return ReturnCode::RESET_FAIL;
        }

        //unimplemented!();//TODO implement (load mem with bios and rom, set registers, etc)
        self.mem_loaded = true;
        return ReturnCode::RESET_OK;
    }

    pub fn tick(self: &mut Self) -> ReturnCode {
        if !self.mem_loaded {
            return ReturnCode::TICK_FAIL;
        }

        //Increment the number of ticks
        self.t += 1;
        log!(self.t, 0, "Tick {} begins", self.t);

        //Fetch from memory
        let mut inst = Inst{wg: [0, 0]};
        if !memory::fetch(self, &mut inst) {
            return ReturnCode::TICK_FAIL_FETCH;
        }

        //Execute the instruction we fetched
        //execute::execute(self,);

        return ReturnCode::TICK_OK;//TODO implement
    }

    pub fn load_bios_file(self: &mut Self, path: &str) -> ReturnCode {
        let load_result = load_file(path, &mut self.bios, MAX_BIOS_SIZE_BYTES);
        if matches!(load_result, ReturnCode::LOAD_OK) {
            self.bios_loaded = true;
        }
        return load_result;
    }

    pub fn load_bios_mem(self: &mut Self, bios_mem: &[u8]) -> ReturnCode {
        unimplemented!();//TODO implement
    }

    pub fn load_rom_file(self: &mut Self, path: &str) -> ReturnCode {
        let load_result = load_file(path, &mut self.rom, MAX_ROM_SIZE_BYTES);
        if matches!(load_result, ReturnCode::LOAD_OK) {
            self.rom_loaded = true;
        }
        return load_result;
    }

    pub fn load_rom_mem(self: &mut Self, rom_mem: &[u8]) -> ReturnCode {
        if rom_mem.len() > MAX_ROM_SIZE_BYTES {
            return ReturnCode::LOAD_FAIL_SIZE;
        }
        unimplemented!();//TODO rom_mem copy into self.rom
        self.rom_loaded = true;
        return ReturnCode::LOAD_OK;
    }
}

/* Functions */

fn load_file(path: &str, buffer: &mut [u8], buffer_size: usize) -> ReturnCode {
    //Open the file
    let file_wrapper = File::open(path);
    if matches!(file_wrapper, Err(_)) {
        return ReturnCode::LOAD_FAIL_OPEN;
    }
    let mut file = file_wrapper.unwrap();

    //Ensure it is not larger than expected
    let metadata_wrapper = file.metadata();
    if matches!(metadata_wrapper, Err(_)) {
        return ReturnCode::LOAD_FAIL_OPEN;
    }
    let metadata = metadata_wrapper.unwrap();
    if metadata.len() > buffer_size.try_into().unwrap() {
        return ReturnCode::LOAD_FAIL_SIZE;
    }

    //Read in its contents into the buffer
    file.read(buffer);
    return ReturnCode::LOAD_OK;
}

fn testing123() {}
