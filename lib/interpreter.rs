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

const MAX_BIOS_SIZE_WORDS: usize = 1 << 22;//FIXME figure out what this actually is
const MAX_ROM_SIZE_WORDS: usize = 1 << 22;//FIXME figure out what this actually is
const MEM_SIZE_WORDS: usize = 1 << 22;
const INT_VECTOR_BASE_ADDR: usize = 0xFFF5;//Page 47 is useful :)
const RESET_VECTOR_ADDR: usize = 0xFFF7;//Page 47 is useful :)

/* Macros */

//TODO (also pub(crate) use the_macro statements here too)

/* Static Variables */

//TODO

/* Types */

pub struct State {
    t: u128,
    regs: Registers,
    buttons: Buttons,
    irq_enabled: bool,
    fiq_enabled: bool,

    bios_loaded: bool,
    rom_loaded: bool,
    mem_loaded: bool,
    bios: Box<[u16]>,
    rom: Box<[u16]>,
    mem: Box<[u16]>,
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

struct SR {//16 bits in all
    //6 bits each
    ds: u8,
    cs: u8,

    //1 bit each
    n: bool,
    z: bool,
    s: bool,
    c: bool,
}

struct Registers {
    sp: u32,//Lower 16 bits is the register, upper 6 is the page
    r: [u16;4],
    bp: u16,
    sr: SR,
    pc: u32,//Lower 16 bits is the register, upper 6 is the page
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

        //TODO return uninited State, since we init it in reset instead
        return State {
            t: 0,
            regs: Registers {
                sp: 0,
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
                pc: 0,
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
            irq_enabled: false,
            fiq_enabled: false,

            /*
            //FIXME use this instead once it is stable
            bios: box [0; MAX_BIOS_SIZE_WORDS],//TODO avoid zero-initializing for speed
            rom: box [0; MAX_ROM_SIZE_WORDS],//TODO avoid zero-initializing for speed
            mem: box [0; MEM_SIZE_WORDS],//TODO avoid zero-initializing for speed
            */
            bios_loaded: false,
            rom_loaded: false,
            mem_loaded: false,
            bios: vec![0u16; MAX_BIOS_SIZE_WORDS].into_boxed_slice(),//TODO avoid vector for speed//TODO avoid zero-initializing for speed
            rom: vec![0u16; MAX_ROM_SIZE_WORDS].into_boxed_slice(),//TODO avoid vector for speed//TODO avoid zero-initializing for speed
            mem: vec![0u16; MEM_SIZE_WORDS].into_boxed_slice(),//TODO avoid vector for speed//TODO avoid zero-initializing for speed
            //TODO other fields
        };
    }

    pub fn reset(self: &mut Self) -> ReturnCode {
        if !self.bios_loaded || !self.rom_loaded {
            return ReturnCode::RESET_FAIL;
        }

        self.t = 0;
        log!(self.t, 0, "Resetting emulated system");


        log!(self.t, 1, "Resetting memory");

        //TEMPORARY for now just copy the bios to the memory
        self.mem.clone_from(&self.bios);
        self.mem_loaded = true;


        log!(self.t, 1, "Resetting CPU registers");

        log!(self.t, 2, "Zero out all registers to begin with");
        self.regs = Registers {
            sp: 0,
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
            pc: 0,
        };
        self.irq_enabled = false;
        self.fiq_enabled = false;

        log!(self.t, 2, "Set initial PC");
        debug_assert!(RESET_VECTOR_ADDR < MEM_SIZE_WORDS);
        log!(self.t, 3, "Read reset vector at address {:#04X}_{:04X}", RESET_VECTOR_ADDR >> 16, RESET_VECTOR_ADDR & 0xFFFF);
        self.regs.pc = self.mem[RESET_VECTOR_ADDR] as u32;//FIXME what about the page?
        log!(self.t, 3, "Initial PC is {:#04X}_{:04X}", self.regs.pc >> 16, self.regs.pc & 0xFFFF);


        //unimplemented!();//TODO implement (load mem with bios and rom, set registers, etc)
        log!(self.t, 0, "Reset complete");
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
        if !execute::execute(self, &inst) {
            return ReturnCode::TICK_FAIL_EXECUTE;
        }

        //TODO rendering, sound, etc

        log!(self.t, 0, "Tick {} ends", self.t);
        return ReturnCode::TICK_OK;
    }

    pub fn load_bios_file(self: &mut Self, path: &str) -> ReturnCode {
        let load_result = load_file(path, &mut self.bios, MAX_BIOS_SIZE_WORDS);
        if matches!(load_result, ReturnCode::LOAD_OK) {
            self.bios_loaded = true;
        }
        return load_result;
    }

    pub fn load_bios_mem(self: &mut Self, bios_mem: &[u16]) -> ReturnCode {
        unimplemented!();//TODO implement
    }

    pub fn load_rom_file(self: &mut Self, path: &str) -> ReturnCode {
        let load_result = load_file(path, &mut self.rom, MAX_ROM_SIZE_WORDS);
        if matches!(load_result, ReturnCode::LOAD_OK) {
            self.rom_loaded = true;
        }
        return load_result;
    }

    pub fn load_rom_mem(self: &mut Self, rom_mem: &[u16]) -> ReturnCode {
        if rom_mem.len() > MAX_ROM_SIZE_WORDS {
            return ReturnCode::LOAD_FAIL_SIZE;
        }
        unimplemented!();//TODO rom_mem copy into self.rom
        self.rom_loaded = true;
        return ReturnCode::LOAD_OK;
    }
}

/* Functions */

fn load_file(path: &str, buffer: &mut [u16], buffer_size: usize) -> ReturnCode {
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
    if metadata.len() > (buffer_size * 2) as u64 {//Ensure it is not too big of a file
        return ReturnCode::LOAD_FAIL_SIZE;
    }
    if (metadata.len() & 0b1) == 0b1 {//Ensure the file is a multiple of 2
        return ReturnCode::LOAD_FAIL_SIZE;
    }

    log!(0, 0, "Loading file \"{}\": {} words | {} bytes", path, metadata.len() / 2, metadata.len());

    //Read in its contents into the buffer
    let mut byte_buffer: Box<[u8]> = vec![0u8; buffer_size * 2].into_boxed_slice();//TODO avoid overhead of zeroing out contents, as well as overhead of needing to copy to buffer instead of reading to it directly
    file.read(&mut byte_buffer);
    for i in 0..buffer_size {
        buffer[i] = ((byte_buffer[(i * 2) + 1] as u16) << 8) | (byte_buffer[i * 2] as u16);
    }
    return ReturnCode::LOAD_OK;
}
