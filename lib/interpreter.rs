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
//!```no_run
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
//!    //Add your own logic (including deciding when to exit and to limit tick()s to 27Mhz) here
//!}
//!```

//TODO we don't _need_ to load a rom; we only need the bios, so we should allow for that
//We also don't need to load the bios; we can also get away with the bios in the rom
//So really we only need one or the other

/* Imports */

mod state;
mod cpu;
mod common;
mod peripherals;

pub use state::State;//TODO make this private once Emulator is implemented

use std::thread;
use std::sync::mpsc::SyncSender;
use std::sync::mpsc::Sender;
use std::sync::mpsc::Receiver;
use std::sync::mpsc::channel;
use std::sync::mpsc::sync_channel;

/* Constants */

const MAX_BIOS_SIZE_WORDS: usize = 1 << 22;//FIXME figure out what this actually is
const MAX_ROM_SIZE_WORDS: usize = 1 << 22;//FIXME figure out what this actually is
const MEM_SIZE_WORDS: usize = 1 << 22;//TODO set this to 0xFFFF since everything above this should not be writable

/* Macros */

//TODO (also pub(crate) use the_macro statements here too)

/* Static Variables */

//TODO

/* Types */

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

pub struct RenderMessage {
    //TODO struct returned by a channel from the renderer containing the data/methods needed to render a frame or access the already rendered frame depending on how things go
}

pub struct SoundMessage {
    //TODO struct returned by a channel from the renderer containing the data/methods indicating how to change the audio being output
}

pub struct InputMessage {
    //TODO message type sent from the user to the channel indicating what to change the state of the inputs to
}

pub struct Emulator {
    state: Option<State>,//We own the state until we launch a thread, at which point the thread owns the state; its ownership is then returned to us when we stop it
    //TODO other fields
    emulation_thread_join_handle: Option<thread::JoinHandle<State>>,
    stop_request_sender: Option<SyncSender<()>>//NOTE: All other channels are part of State, except for this one which is just used internally to request the thread to stop
}

/* Associated Functions and Methods */

impl Emulator {
    pub fn new() -> Emulator {
        return Emulator {
            state: Some(State::new()),
            emulation_thread_join_handle: None,
            stop_request_sender: None,
        };
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

    pub fn launch_emulation_thread(self: &mut Self)  {
        debug_assert!(matches!(self.stop_request_sender, None));
        debug_assert!(!matches!(self.state, None));

        //Create the channel for requesting stops later
        let (tx, rx) = sync_channel::<()>(0);
        self.stop_request_sender.replace(tx);

        //Get the state to give to the thread (the state in our struct becomes None due to take())
        let state_for_thread = self.state.take().unwrap();

        //Launch the thread
        self.emulation_thread_join_handle.replace(thread::spawn(move || -> State { return Emulator::emulation_thread(state_for_thread, rx); }));
    }

    pub fn stop_emulation_thread(self: &mut Self) {
        debug_assert!(!matches!(self.stop_request_sender, None));
        debug_assert!(!matches!(self.emulation_thread_join_handle, None));

        //Request the thread to stop and get the state back from it; also destroy the stop request channel
        let moved_stop_request_sender = self.stop_request_sender.take().unwrap();
        moved_stop_request_sender.send(());

        let old_join_handle = self.emulation_thread_join_handle.take().unwrap();
        let state_from_thread = old_join_handle.join().unwrap();

        drop(moved_stop_request_sender);

        //Replace the state in our Emulator struct so we can restart it again later
        self.state.replace(state_from_thread);
    }

    fn emulation_thread(state: State, stop_request_reciever: Receiver<()>) -> State {
        while true {
            //Check if we've recieved a request to exit, and if so, break out of the loop
            if matches!(stop_request_reciever.try_recv(), Ok(_)) {
                break;
            }

            //TODO
            todo!();
        }

        return state;//Give the state back when we're finished with it
    }
}

/* Functions */

//TODO
