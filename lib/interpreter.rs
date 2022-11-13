/* interpreter.rs: VSEMUR Interpreter
 * By: John Jekel
 *
 * Emulates a VSmile system one .tick() at a time!
 *
*/

//!VSEMUR Interpreter
//!
//!By: John Jekel
//!
//!Emulates a VSmile system one .tick() at a time!
//!
//!# Example usage
//!
//!```
//!use vsemur::interpreter;
//!
//!//Initialize state
//!let mut state: interpreter::State = interpreter::State::new();
//!
//!//Load bios and rom
//!if !matches!(state.load_bios_file("path/to/bios.bin"), interpreter::ReturnCode::LoadOk) {
//!    panic!("Error: Failed to load bios from disk");
//!}
//!if !matches!(state.load_rom_file("path/to/rom.bin"), interpreter::ReturnCode::LoadOk) {
//!    panic!("Error: Failed to load rom from disk");
//!}
//!
//!//Power-on reset
//!if !matches!(state.reset(), interpreter::ReturnCode::ResetOk) {
//!    panic!("Error: Reset failed");
//!}
//!
//!//Main emulation loop
//!loop {
//!    match state.tick() {
//!        interpreter::ReturnCode::TickOk => { /* No special handling needed */ },
//!        interpreter::ReturnCode::TickFail => {
//!            if cfg!(debug_assertions) {
//!                panic!("Error: Tick failed");
//!            }
//!        },
//!        interpreter::ReturnCode::TickOkNewFrameAvailable => {
//!            //Add your own logic here to display the new frame to the user
//!        }
//!        _ => {
//!            panic!("This will never occur");
//!        },
//!    }
//!    //Add your own logic (including deciding when to exit) here
//!}
//!```

//TODO we don't _need_ to load a rom; we only need the bios, so we should allow for that

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

///State for the VSEMUR interpreter
///
///Holds all information needed to store the state of an emulated VSmile system.
///
///Instanciate with [`State::new()`].
pub struct State {
    t: u128,//Ticks

    cpu: cpu::CPUState,
    render: render::RenderState,
    sound: sound::SoundState,
    input: input::InputState,
    mem: memory::MemoryState,
}

///Return type for several VSEMUR interpreter functions
pub enum ReturnCode {
    ///The call to [`State::tick()`] was sucessful, no additional action is required.
    TickOk,
    ///The call to [`State::tick()`] failed for some reason.
    TickFail,
    ///The call to [`State::tick()`] was sucessful, and additionally a new frame is available to be displayed.
    TickOkNewFrameAvailable,

    ///The call to [`State::reset()`] was sucessful.
    ResetOk,
    ///The call to [`State::reset()`] failed for some reason.
    ResetFail,

    ///The call to [`State::load_bios_file()`], [`State::load_bios_mem()`], [`State::load_rom_file()`], or [`State::load_rom_mem()`] was sucessful.
    LoadOk,
    ///The call to [`State::load_bios_file()`] or [`State::load_rom_file()`] failed due to a filesystem issue.
    LoadFailOpen,
    ///The call to [`State::load_bios_file()`], [`State::load_bios_mem()`], [`State::load_rom_file()`], or [`State::load_rom_mem()`] failed due to the source being an invalid size.
    LoadFailSize,
}

/* Associated Functions and Methods */

//Only functions called by external users are associated functions/methods
//Everything else goes into other modules and are not associated
impl State {
    ///Instanciates a new [`State`].
    ///
    ///You probably want to load a rom and bios after this; see [`State::load_bios_file()`], [`State::load_bios_mem()`], [`State::load_rom_file()`], and [`State::load_rom_mem()`].
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

    ///Resets the emulated system.
    ///
    ///Requires that a rom and bios have already been loaded beforehand; see [`State::load_bios_file()`], [`State::load_bios_mem()`], [`State::load_rom_file()`], and [`State::load_rom_mem()`].
    ///
    ///Returns [`ReturnCode::ResetFail`] if a BIOS or ROM wasn't loaded beforehand; otherwise returns [`ReturnCode::ResetOk`].
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

    ///Performs one "tick" of the emulated system, equivalent to one clock cycle.
    ///
    ///This function should be called approximately (TODO determine the proper clock frequency) times per second
    ///
    ///Before this is called, [`State::reset()`] should already have been called at least once.
    ///
    ///Returns [`ReturnCode::TickFail`] if the proper prerequisites have not been met. Otherwise normally returns [`ReturnCode::TickOk`], unless a new frame is ready to be shown to the user, in which case it returns [`ReturnCode::TickOkNewFrameAvailable`].
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

    ///Loads a VSmile BIOS file from disk at the path specified.
    ///
    ///After this function is called, [`State::reset()`] must be called before [`State::tick()`] is called again.
    ///
    ///Returns [`ReturnCode::LoadOk`] if the load was sucessful, [`ReturnCode::LoadFailOpen`] if there was a filesystem issue, [`ReturnCode::LoadFailSize`] if the file was an invalid size.
    pub fn load_bios_file(self: &mut Self, path: &str) -> ReturnCode {
        return self.mem.load_bios_file(path);
    }

    ///Loads a VSmile BIOS from the memory contained within the given slice.
    ///
    ///After this function is called, [`State::reset()`] must be called before [`State::tick()`] is called again.
    ///
    ///Returns [`ReturnCode::LoadOk`] if the load was sucessful, or [`ReturnCode::LoadFailSize`] if the slice was an invalid size.
    pub fn load_bios_mem(self: &mut Self, bios_mem: &[u16]) -> ReturnCode {
        return self.mem.load_bios_mem(bios_mem);
    }

    ///Loads a VSmile rom file from disk at the path specified.
    ///
    ///After this function is called, [`State::reset()`] must be called before [`State::tick()`] is called again.
    ///
    ///Returns [`ReturnCode::LoadOk`] if the load was sucessful, [`ReturnCode::LoadFailOpen`] if there was a filesystem issue, [`ReturnCode::LoadFailSize`] if the file was an invalid size.
    pub fn load_rom_file(self: &mut Self, path: &str) -> ReturnCode {
        return self.mem.load_rom_file(path);
    }

    ///Loads a VSmile rom from the memory contained within the given slice.
    ///
    ///After this function is called, [`State::reset()`] must be called before [`State::tick()`] is called again.
    ///
    ///Returns [`ReturnCode::LoadOk`] if the load was sucessful, or [`ReturnCode::LoadFailSize`] if the slice was an invalid size.
    pub fn load_rom_mem(self: &mut Self, rom_mem: &[u16]) -> ReturnCode {
        return self.mem.load_rom_mem(rom_mem);
    }

    //TODO functions to read frambuffer so the user can display it as they wish
}

/* Functions */

//TODO
