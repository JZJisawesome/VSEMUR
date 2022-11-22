/* NAME//TODO
 * By: John Jekel
 *
 * TODO description
 *
*/

/* Imports */

use super::RenderMessage;
use super::SoundMessage;
use super::InputMessage;
use super::ReturnCode;
use super::cpu::CPUState;
use super::peripherals::Peripherals;

use crate::debug_panic;

use crate::logging::log;
use crate::logging::log_ansi;
use crate::logging::log_increment_ticks;//TODO increment ticks in Emulator instead

use std::thread;
use std::sync::mpsc::Sender;
use std::sync::mpsc::Receiver;
use std::sync::mpsc::channel;

/* Constants */

//TODO deal with banking

//All inclusive


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
    cpu: CPUState,
    peripherals: Peripherals,
}

/* Associated Functions and Methods */

//Public methods
impl State {
    ///Instanciates a new [`State`].
    ///
    ///You probably want to load a rom and bios after this; see [`State::load_bios_file()`], [`State::load_bios_mem()`], [`State::load_rom_file()`], and [`State::load_rom_mem()`].
    pub fn new() -> State {
        let new_state = State {
            cpu: CPUState::new(),
            peripherals: Peripherals::new(),
        };

        log!(0, "Initialization complete");
        return new_state;
    }

    ///Resets the emulated system.
    ///
    ///Requires that a rom and bios have already been loaded beforehand; see [`State::load_bios_file()`], [`State::load_bios_mem()`], [`State::load_rom_file()`], and [`State::load_rom_mem()`].
    ///
    ///Returns [`ReturnCode::ResetFail`] if a BIOS or ROM wasn't loaded beforehand; otherwise returns [`ReturnCode::ResetOk`].
    pub fn reset(self: &mut Self) -> ReturnCode {
        log_ansi!(0, "\x1b[1;97m", "Resetting emulated system");

        self.peripherals.reset();//Must come before the CPU so that it can fetch the reset vector, etc
        self.cpu.reset(&mut self.peripherals);

        log!(0, "Reset complete");
        return ReturnCode::ResetOk;
    }

    /*pub fn cache(self: &mut Self) {
        self.cpu.cache(&mut self.mem);
    }*/

    ///Performs one "tick" of the emulated system, equivalent to one clock cycle.
    ///
    ///This function should be called approximately 27 million times per second (27 MHz)
    ///
    ///Before this is called, [`State::reset()`] should already have been called at least once.
    ///
    ///Returns [`ReturnCode::TickFail`] if the proper prerequisites have not been met. Otherwise normally returns [`ReturnCode::TickOk`], unless a new frame is ready to be shown to the user, in which case it returns [`ReturnCode::TickOkNewFrameAvailable`].
    pub fn tick(self: &mut Self) -> ReturnCode {
        //Increment the number of ticks for debugging
        log_increment_ticks!();
        log_ansi!(0, "\x1b[1;97m", "Tick begins");

        //TODO instead of calling this function at 27MHz, run a certain number of cycles per frame (whatever number would run on the hardware at 60Hz) and limit the speed that way
        //This is what other emulators do, and it a good idea for you to do too
        //https://stackoverflow.com/questions/112439/cpu-emulation-and-locking-to-a-specific-clock-speed

        //Tick sub-states
        self.cpu.tick(&mut self.peripherals);
        self.peripherals.tick();

        log!(0, "Tick ends");
        return ReturnCode::TickOk;
    }

    pub fn get_render_reciever(self: &mut Self) -> Receiver<RenderMessage> {
        todo!();//Construct the channel here and pass it to the State object through methods we'll have to add
    }

    pub fn get_sound_reciever(self: &mut Self) -> Receiver<SoundMessage> {
        todo!();//Construct the channel here and pass it to the State object through methods we'll have to add
    }

    pub fn get_input_sender(self: &mut Self) -> Sender<InputMessage> {
        todo!();//Construct the channel here and pass it to the State object through methods we'll have to add
    }

    pub(super) fn ready(self: &Self) -> bool {
        return true;//TODO
    }

    ///Loads a VSmile BIOS file from disk at the path specified.
    ///
    ///After this function is called, [`State::reset()`] must be called before [`State::tick()`] is called again.
    ///
    ///Returns [`ReturnCode::LoadOk`] if the load was sucessful, [`ReturnCode::LoadFailOpen`] if there was a filesystem issue, [`ReturnCode::LoadFailSize`] if the file was an invalid size.
    pub fn load_bios_file(self: &mut Self, path: &str) -> ReturnCode {
        return self.peripherals.load_bios_file(path);
    }

    ///Loads a VSmile BIOS from the memory contained within the given slice.
    ///
    ///After this function is called, [`State::reset()`] must be called before [`State::tick()`] is called again.
    ///
    ///Returns [`ReturnCode::LoadOk`] if the load was sucessful, or [`ReturnCode::LoadFailSize`] if the slice was an invalid size.
    pub fn load_bios_mem(self: &mut Self, bios_mem: &[u16]) -> ReturnCode {
        return self.peripherals.load_bios_mem(bios_mem);
    }

    ///Loads a VSmile rom file from disk at the path specified.
    ///
    ///After this function is called, [`State::reset()`] must be called before [`State::tick()`] is called again.
    ///
    ///Returns [`ReturnCode::LoadOk`] if the load was sucessful, [`ReturnCode::LoadFailOpen`] if there was a filesystem issue, [`ReturnCode::LoadFailSize`] if the file was an invalid size.
    pub fn load_rom_file(self: &mut Self, path: &str) -> ReturnCode {
        return self.peripherals.load_rom_file(path);
    }

    ///Loads a VSmile rom from the memory contained within the given slice.
    ///
    ///After this function is called, [`State::reset()`] must be called before [`State::tick()`] is called again.
    ///
    ///Returns [`ReturnCode::LoadOk`] if the load was sucessful, or [`ReturnCode::LoadFailSize`] if the slice was an invalid size.
    pub fn load_rom_mem(self: &mut Self, rom_mem: &[u16]) -> ReturnCode {
        return self.peripherals.load_rom_mem(rom_mem);
    }

    //TODO I feel like these interfaces aren't the best design; we should just pass the channel to State directly
    //fn get_render_message(self: &mut Self) -> Option<RenderMessage>//TODO if this is not None then we both send the message down the channel in Emulation and wait until the next frame
    //fn get_sound_message(self: &mut Self) -> Option<SoundMessage>
    //fn set_input_message(self: &mut Self) -> Option<InputMessage>
    //TODO perhaps register closures as callbacks, which send a message in Emulation? That dosn't really work either

    //TODO functions to read frambuffer so the user can display it as they wish
    //Perhaps provide some sort of thread-safe function so we can render the screen asynchrenously?
    //Don't actually do rendering in render::tick(), just save the data we need to render later, and don't actually render until the user
    //calls our function to do so (which could be asynchrenous)
    //Idea:
    //State::latch_render() copies the values/data internally needed to render later (meaning we can tick right away afterwards since even if we affect the render settings we have a copy)
    // Or perhaps this returns some sort of FutureFramebuffer object that we can call a render function on, completely seperate from the main State?
    //State::render() returns the rendered framebuffer (the user can call this in a seperate thread)
}

/* Functions */
