/* NAME//TODO
 * By: John Jekel
 *
 * TODO description
 *
*/

/* Imports */

use crate::interpreter::cpu;
use crate::interpreter::render;
use crate::interpreter::sound;
use crate::interpreter::input;
//use crate::interpreter::execute;
use crate::interpreter::memory;

use crate::logging::log;

/* Constants */

pub(super) const MAX_BIOS_SIZE_WORDS: usize = 1 << 22;//FIXME figure out what this actually is
pub(super) const MAX_ROM_SIZE_WORDS: usize = 1 << 22;//FIXME figure out what this actually is
pub(super) const MEM_SIZE_WORDS: usize = 1 << 22;
pub(super) const INT_VECTOR_BASE_ADDR: usize = 0xFFF5;//Page 47 is useful :)
pub(super) const RESET_VECTOR_ADDR: usize = 0xFFF7;//Page 47 is useful :)

/* Macros */

//TODO (also pub(crate) use the_macro statements here too)

/* Static Variables */

//TODO

/* Types */

pub struct State {
    t: u128,

    cpu: cpu::CPUState,
    render: render::RenderState,
    sound: sound::SoundState,
    input: input::InputState,
    mem: memory::MemoryState,
}

pub enum ReturnCode {
    TICK_OK,
    TICK_FAIL,

    RESET_OK,
    RESET_FAIL,

    LOAD_OK,
    LOAD_FAIL_OPEN,
    LOAD_FAIL_SIZE,

}


pub struct ControllerButtons {
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

pub struct Buttons {
    p1: ControllerButtons,
    p2: ControllerButtons,
}

pub(super) struct Inst {
    pub(super) wg: [u16; 2],
}

/* Associated Functions and Methods */

//Only functions called by external users are associated functions/methods
//Everything else goes into other modules and are not associated
impl State {
    pub fn new() -> State {
        log!(0, 0, "Initializing VSEMUR State");

        return State {
            t: 0,
            cpu: cpu::CPUState::new(),
            render: render::RenderState::new(),
            sound: sound::SoundState::new(),
            input: input::InputState::new(),
            mem: memory::MemoryState::new(),
        };
    }

    pub fn reset(self: &mut Self) -> ReturnCode {
        if !self.mem.ready() {
            return ReturnCode::RESET_FAIL;
        }

        self.t = 0;
        log!(self.t, 0, "Resetting emulated system");

        self.cpu.reset();
        self.render.reset();
        self.sound.reset();
        self.input.reset();
        if !self.mem.reset() {
            return ReturnCode::RESET_FAIL;
        }

        log!(self.t, 0, "Reset complete");
        return ReturnCode::RESET_OK;

        /*if !self.bios_loaded || !self.rom_loaded {
            return ReturnCode::RESET_FAIL;
        }


        log!(self.t, 1, "Resetting memory");


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

        log!(self.t, 2, "Set initial CS page and PC");
        debug_assert!(RESET_VECTOR_ADDR < MEM_SIZE_WORDS);
        log!(self.t, 3, "Read reset vector at address {:#04X}_{:04X}", RESET_VECTOR_ADDR >> 16, RESET_VECTOR_ADDR & 0xFFFF);
        self.regs.pc = self.mem[RESET_VECTOR_ADDR];
        log!(self.t, 3, "Initial CS page, PC is {:#04X}_{:04X}", self.regs.sr.cs, self.regs.pc);

        //TODO do we need to initialize the cs or ds?
        */

        //unimplemented!();//TODO implement (load mem with bios and rom, set registers, etc)
    }

    pub fn tick(self: &mut Self) -> ReturnCode {
        if !self.mem.ready() {
            return ReturnCode::TICK_FAIL;
        }

        //Increment the number of ticks
        self.t += 1;
        log!(self.t, 0, "\x1b[1;97mTick {} begins\x1b[0m", self.t);

        //Tick sub-states
        /*cpu.tick();
        render.tick();
        sound.tick();
        input.tick();
        */

        /*if !self.mem.bios_loaded || !self.mem.rom_loaded || !self.mem_loaded {
            return ReturnCode::TICK_FAIL;
        }




        //FIXME
        //FIXME
        //Create member function to access the PC to support auto increment WITH paging correctly

        //Fetch from memory
        let mut inst = Inst{wg: [0, 0]};//TODO
        if !memory::fetch(self, &mut inst) {
            return ReturnCode::TICK_FAIL_FETCH;
        }

        //Execute the instruction we fetched
        if !execute::execute(self, &inst) {
            return ReturnCode::TICK_FAIL_EXECUTE;
        }
        */

        //TODO rendering, sound, etc

        log!(self.t, 0, "Tick {} ends", self.t);
        return ReturnCode::TICK_OK;
    }
    pub fn load_bios_file(self: &mut Self, path: &str) -> ReturnCode {
        return self.mem.load_bios_file(path);
    }

    pub fn load_bios_mem(self: &mut Self, bios_mem: &[u16]) -> ReturnCode {
        return self.mem.load_bios_mem(bios_mem);
    }

    pub fn load_rom_file(self: &mut Self, path: &str) -> ReturnCode {
        return self.mem.load_rom_file(path);
    }

    pub fn load_rom_mem(self: &mut Self, rom_mem: &[u16]) -> ReturnCode {
        return self.mem.load_rom_mem(rom_mem);
    }
}

/* Functions */
