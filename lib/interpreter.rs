/* interpreter.rs: VSEMUR Interpreter
 * By: John Jekel
 *
 * Emulates a VSmile system one .tick() at a time!
 *
*/

/* Imports */

mod cpu;
mod input;
mod memory;
mod render;
mod sound;

use crate::logging::log;
use crate::logging::log_ansi;
use crate::logging::log_reset_file;

/* Constants */

const MAX_BIOS_SIZE_WORDS: usize = 1 << 22;//FIXME figure out what this actually is
const MAX_ROM_SIZE_WORDS: usize = 1 << 22;//FIXME figure out what this actually is
const MEM_SIZE_WORDS: usize = 1 << 22;//TODO set this to 0xFFFF since everything above this should not be writable

/* Macros */

//TODO (also pub(crate) use the_macro statements here too)

/* Static Variables */

//TODO

/* Types */

pub struct State {
    t: u128,//Ticks

    cpu: cpu::CPUState,
    render: render::RenderState,
    sound: sound::SoundState,
    input: input::InputState,
    mem: memory::MemoryState,
}

pub enum ReturnCode {
    TickOk,
    TickFail,

    ResetOk,
    ResetFail,

    LoadOk,
    LoadFailOpen,
    LoadFailSize,
}

/* Associated Functions and Methods */

//Only functions called by external users are associated functions/methods
//Everything else goes into other modules and are not associated
impl State {
    pub fn new() -> State {
        log_reset_file!();

        log_ansi!(0, 0, "\x1b[1;97m", "Initializing VSEMUR State");

        let new_state = State {
            t: 0,
            cpu: cpu::CPUState::new(),
            render: render::RenderState::new(),
            sound: sound::SoundState::new(),
            input: input::InputState::new(),
            mem: memory::MemoryState::new(),
        };

        log!(0, 0, "Initialization complete");

        return new_state
    }

    pub fn reset(self: &mut Self) -> ReturnCode {
        self.t = 0;
        log_ansi!(self.t, 0, "\x1b[1;97m", "Resetting emulated system");

        //Memory must be reset first since other parts may depend on values in it at reset
        if !self.mem.reset() {//BIOS or ROM wasn't loaded
            return ReturnCode::ResetFail;
        }

        self.cpu.reset(&mut self.mem);
        self.render.reset();
        self.sound.reset();
        self.input.reset();

        log!(self.t, 0, "Reset complete");
        return ReturnCode::ResetOk;
    }

    pub fn tick(self: &mut Self) -> ReturnCode {
        if !self.mem.ready() {
            return ReturnCode::TickFail;
        }

        //Increment the number of ticks
        self.t += 1;
        log_ansi!(self.t, 0, "\x1b[1;97m", "Tick {} begins", self.t);

        //Tick sub-states
        self.cpu.tick(self.t, &mut self.mem);
        self.render.tick();
        self.sound.tick();

        log!(self.t, 0, "Tick {} ends", self.t);
        return ReturnCode::TickOk;
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

    //TODO functions to read frambuffer so the user can display it as they wish
}

/* Functions */

//TODO
