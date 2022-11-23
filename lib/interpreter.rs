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
//!TODO redo this using Emulator instead
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

use std::thread;
use std::sync::mpsc::SyncSender;
use std::sync::mpsc::Sender;
use std::sync::mpsc::Receiver;
use std::sync::mpsc::channel;
use std::sync::mpsc::sync_channel;


use crate::logging::log;
use crate::logging::log_ansi;
use crate::logging::log_reset_file;
use crate::logging::log_reset_ticks;

/* Constants */

//TODO

/* Macros */

//TODO (also pub(crate) use the_macro statements here too)

/* Static Variables */

//TODO

/* Types */

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
    state: Option<state::State>,//We own the state until we launch a thread, at which point the thread owns the state; its ownership is then returned to us when we stop it
    //TODO other fields
    emulation_thread_join_handle: Option<thread::JoinHandle<state::State>>,
    stop_request_sender: Option<SyncSender<()>>//NOTE: All other channels are part of State, except for this one which is just used internally to request the thread to stop
}

/* Associated Functions and Methods */

///VSEMUR Interpreter primary emulation struct
///
///Holds all information needed to store the state of an emulated VSmile system, in addition to data to manage threading and message-passing
///
///Instanciate with [`Emulator::new()`].
impl Emulator {
    ///Instanciates a new [`Emulator`].
    ///
    ///You probably want to load a BIOS/ROM after this; see [`Emulator::load_bios_file()`], [`Emulator::load_bios_mem()`], [`Emulator::load_rom_file()`], and [`Emulator::load_rom_mem()`].
    pub fn new() -> Emulator {
        log_reset_file!();
        log_reset_ticks!();

        log_ansi!(0, "\x1b[1;97m", "Initializing VSEMUR Emulator");

        return Emulator {
            state: Some(state::State::new()),
            emulation_thread_join_handle: None,
            stop_request_sender: None,
        };
    }

    ///Returns `true` if the emulation thread associated with this [`Emulator`] is currently running, and false otherwise
    pub fn thread_running(self: &Self) -> bool {
        debug_assert!(matches!(self.state, None) != matches!(self.emulation_thread_join_handle, None));
        debug_assert!(matches!(self.emulation_thread_join_handle, None) == matches!(self.stop_request_sender, None));
        return matches!(self.state, None);
    }

    //TODO add mega setup function that calls load functions and returns the reciever

    ///Resets the emulated system (sets registers to default values, sets the CPU's initial PC, and so on).
    ///
    ///The emulation thread must not be running when this is called ([`Emulator::thread_running()`] must return false).
    ///
    ///You likely want to at least have a BIOS loaded before calling this, as it will be accessed by this function to properly initialize things
    pub fn reset(self: &mut Self) {//Must be called after loading is complete (does not care if channels are sent yet)
        debug_assert!(!self.thread_running());
        self.state.as_mut().unwrap().reset();
    }

    //TODO these will be valid across launches and stops of the emulation thread, but can be called whenever we're stopped to recreate them if needed
    pub fn get_render_reciever(self: &mut Self) -> Receiver<RenderMessage> {
        debug_assert!(!self.thread_running());
        return self.state.as_mut().unwrap().get_render_reciever();
    }

    pub fn get_sound_reciever(self: &mut Self) -> Receiver<SoundMessage> {
        debug_assert!(!self.thread_running());
        return self.state.as_mut().unwrap().get_sound_reciever();
    }

    pub fn get_input_sender(self: &mut Self) -> Sender<InputMessage> {
        debug_assert!(!self.thread_running());
        return self.state.as_mut().unwrap().get_input_sender();
    }

    ///Loads a VSmile BIOS file from disk at the path specified.
    ///
    ///The emulation thread must not be running when this is called ([`Emulator::thread_running()`] must return false).
    ///
    ///After this function is called, [`Emulator::reset()`] must be called before the emulation thread is launched with [`Emulator::launch_emulation_thread()`].
    ///
    ///Returns `Result::Ok(())` if the load was sucessful, or otherwise `Result::Err(())` if it was not.
    pub fn load_bios_file(self: &mut Self, path: &str) -> Result<(), ()> {
        debug_assert!(!self.thread_running());
        return self.state.as_mut().unwrap().load_bios_file(path);
    }

    ///Loads a VSmile BIOS from the memory contained within the given slice.
    ///
    ///The emulation thread must not be running when this is called ([`Emulator::thread_running()`] must return false).
    ///
    ///After this function is called, [`Emulator::reset()`] must be called before the emulation thread is launched with [`Emulator::launch_emulation_thread()`].
    ///
    ///Returns `Result::Ok(())` if the load was sucessful, or otherwise `Result::Err(())` if it was not.
    pub fn load_bios_mem(self: &mut Self, bios_mem: &[u16]) -> Result<(), ()> {
        debug_assert!(!self.thread_running());
        return self.state.as_mut().unwrap().load_bios_mem(bios_mem);
    }

    ///Loads a VSmile rom file from disk at the path specified.
    ///
    ///The emulation thread must not be running when this is called ([`Emulator::thread_running()`] must return false).
    ///
    ///After this function is called, [`Emulator::reset()`] must be called before the emulation thread is launched with [`Emulator::launch_emulation_thread()`].
    ///
    ///Returns `Result::Ok(())` if the load was sucessful, or otherwise `Result::Err(())` if it was not.
    pub fn load_rom_file(self: &mut Self, path: &str) -> Result<(), ()> {
        debug_assert!(!self.thread_running());
        return self.state.as_mut().unwrap().load_rom_file(path);
    }

    ///Loads a VSmile rom from the memory contained within the given slice.
    ///
    ///The emulation thread must not be running when this is called ([`Emulator::thread_running()`] must return false).
    ///
    ///After this function is called, [`Emulator::reset()`] must be called before the emulation thread is launched with [`Emulator::launch_emulation_thread()`].
    ///
    ///Returns `Result::Ok(())` if the load was sucessful, or otherwise `Result::Err(())` if it was not.
    pub fn load_rom_mem(self: &mut Self, rom_mem: &[u16]) -> Result<(), ()> {
        debug_assert!(!self.thread_running());
        return self.state.as_mut().unwrap().load_rom_mem(rom_mem);
    }

    ///Launches a new emulation thread and associates it with this Emulator
    ///
    ///Requires that a BIOS (and optionally ROM) has been loaded, that the Emulator has been reset at least once, and that the render and sound recievers,
    ///as well as the sound sender get functions have been called and are set to be monitored in the user's code (in seperate threads or a single one per
    ///per the user's preferences and design choice).
    ///
    ///IT IS IMPERATIVE THAT THE RENDER AND SOUND RECIEVERS AND INPUT SENDER ASSOCIATED WITH THIS EMULATOR ARE NOT DROPPED WHILE THE THREAD IS RUNNING.
    ///
    ///See [`Emulator::stop_emulation_thread()`] for how to stop the running thread.
    pub fn launch_emulation_thread(self: &mut Self) {
        debug_assert!(!self.thread_running());

        log_ansi!(0, "\x1b[1;97m", "Starting emulation thread");

        //Reset log ticks, but don't reset the log file
        log_reset_ticks!();

        //Create the channel for requesting stops later
        let (tx, rx) = sync_channel::<()>(0);
        self.stop_request_sender.replace(tx);

        //Get the state to give to the thread (the state in our struct becomes None due to take())
        let state_for_thread = self.state.take().unwrap();
        debug_assert!(state_for_thread.ready());

        //Launch the thread
        self.emulation_thread_join_handle.replace(thread::spawn(move || -> state::State { return Emulator::emulation_thread(state_for_thread, rx); }));
    }

    ///Stops the currently running thread, blocking until it finishes and exits.
    ///
    ///This function should only be called if the emulation thread is actually running ([`Emulator::thread_running()`] must return true).
    ///
    ///Depending on the timing of this call, the function may block for an entire frame's worth of time. Keep this in mind.
    pub fn stop_emulation_thread(self: &mut Self) {
        debug_assert!(self.thread_running());

        log_ansi!(0, "\x1b[1;97m", "Stopping emulation thread via friendly request");

        //Request the thread to stop and get the state back from it; also destroy the stop request channel
        let moved_stop_request_sender = self.stop_request_sender.take().unwrap();
        if matches!(moved_stop_request_sender.send(()), Err(_)) {
            panic!("Emulation thread dropped reciever (likely due to panic)");
        }

        let old_join_handle = self.emulation_thread_join_handle.take().unwrap();
        let state_from_thread = old_join_handle.join().expect("Emulation thread panicked");

        drop(moved_stop_request_sender);

        //Replace the state in our Emulator struct so we can restart it again later
        self.state.replace(state_from_thread);

        log_ansi!(0, "\x1b[1;97m", "Emulation thread stopped");
    }

    fn emulation_thread(mut state: state::State, stop_request_reciever: Receiver<()>) -> state::State {
        log_ansi!(0, "\x1b[1;97m", "Emulation thread started");

        //Constants//TODO move these elsewhere
        const INSTS_PER_FRAME: usize = 450000;
        const ENABLE_EFFICIENT_SLEEP: bool = false;
        const BUSY_WAIT_YIELD: bool = true;
        const FRAME_PERIOD: std::time::Duration = std::time::Duration::from_nanos(16666667);//1/60th of a second
        const BUSY_WAIT_TIME_PER_FRAME: std::time::Duration = std::time::Duration::from_micros(500);//Larger values waste more CPU time, but if this is too small we may feel the effects of the thread's wake up latency

        //The frame loop
        loop {
            let start_of_frame = std::time::Instant::now();

            //Check if we've recieved a request to exit, and if so, break out of the loop
            //Checking this once per frame is less expensive than once per emulated clock cycle
            if matches!(stop_request_reciever.try_recv(), Ok(())) {
                log_ansi!(0, "\x1b[1;97m", "Emulation thread stop request recieved");
                break;
            }

            for _ in 0..INSTS_PER_FRAME {
                state.tick();
                if state.frame_ended() {//We want to sync the number of ticks we perform with actual frames, not just use frames as a measure of rate-limiting
                    break;
                }
            }

            let frame_time = start_of_frame.elapsed();

            //TESTING print the frame time//TODO perhaps save this value somewhere where the user can access it later?
            eprint!("frametime: {}ns, ", frame_time.as_nanos());

            if frame_time < FRAME_PERIOD {
                if ENABLE_EFFICIENT_SLEEP {
                    if (FRAME_PERIOD - frame_time) > BUSY_WAIT_TIME_PER_FRAME {
                        //This causes worse frame times, not just due to wake-up latency; having a less-effective CPU cache when we recieve control back slows things down
                        std::thread::sleep(FRAME_PERIOD - frame_time - BUSY_WAIT_TIME_PER_FRAME);
                    }
                }

                if BUSY_WAIT_YIELD {
                    //Busy wait for the remaining time (deals with the wakeup latency of sleeping; also does not really impact CPU caches); with yield to lessen the impact on CPU usage
                    while start_of_frame.elapsed() < FRAME_PERIOD { std::thread::yield_now(); }
                } else {
                    //Alternative: busy-waiting without yield (best performance and frame pacing, but will mean the emulation thread always pins its core to 100%)
                    while start_of_frame.elapsed() < FRAME_PERIOD {}
                }
            } else {
                //We're either early or we're late!
                eprintln!("\x1b[31mWarning: emulation thread not fast enough, frame took {}ns\x1b[0m", frame_time.as_nanos());
            }

            let rate_limited_frame_time = start_of_frame.elapsed();

            //TESTING
            eprintln!("rate-limited: {}ns, ", rate_limited_frame_time.as_nanos());
        }

        return state;//Give the state back when we're finished with it
    }
}

/* Functions */

//TODO
