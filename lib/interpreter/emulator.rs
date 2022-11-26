/* emulator.rs
 * By: John Jekel
 *
 * The Emulator struct holds all information needed to store the state of an emulated
 * VSmile system, in addition to data to manage threading and message-passing
*/

//TODO remove this once everything is implemented
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(dead_code)]

/* Imports */

use super::unsp;
use super::render_reciever::RenderReciever;
use super::sound_reciever::SoundReciever;
use super::input_sender::InputSender;
use super::state::State;

use std::thread;
use std::sync::mpsc::SyncSender;
use std::sync::mpsc::Receiver;
use std::sync::mpsc::sync_channel;

use crate::logging::log;
use crate::logging::log_ansi;
use crate::logging::log_reset_file;
use crate::logging::log_reset_ticks;
use crate::logging::log_increment_ticks;

/* Constants */

const CYCLES_PER_FRAME: usize = 450000;
const ENABLE_EFFICIENT_SLEEP: bool = false;
const BUSY_WAIT_YIELD: bool = true;
const FRAME_PERIOD: std::time::Duration = std::time::Duration::from_nanos(16666667);//1/60th of a second
const BUSY_WAIT_TIME_PER_FRAME: std::time::Duration = std::time::Duration::from_micros(500);//Larger values waste more CPU time, but if this is too small we may feel the effects of the thread's wake up latency

/* Macros */

//TODO (also pub(crate) use the_macro statements here too)

/* Static Variables */

//TODO

/* Types */

///VSEMUR Interpreter primary emulation struct
///
///Holds all information needed to store the state of an emulated VSmile system, in addition to data to manage threading and message-passing
///
///Instanciate with [`Emulator::new()`].
pub struct Emulator {
    //We own the state until we launch a thread, at which point the thread owns it; its ownership is then returned to us when we stop it
    state: Option<State>,

    //TODO other fields
    emulation_thread_join_handle: Option<thread::JoinHandle<State>>,
    stop_request_sender: Option<SyncSender<()>>//NOTE: All other channels are part of Peripherals, except for this one which is just used internally to request the thread to stop
}

/* Associated Functions and Methods */

impl Emulator {
    ///Instanciates a new [`Emulator`].
    ///
    ///You probably want to load a BIOS/ROM after this; see [`Emulator::load_bios_file()`], [`Emulator::load_bios_mem()`], [`Emulator::load_rom_file()`], and [`Emulator::load_rom_mem()`].
    pub fn new() -> Emulator {
        log_reset_file!();
        log_reset_ticks!();

        log_ansi!(0, "\x1b[1;97m", "Initializing VSEMUR Emulator");

        let new_emulator = Emulator {
            state: Some(State::new()),
            emulation_thread_join_handle: None,
            stop_request_sender: None,
        };

        log!(0, "Initialization complete");
        return new_emulator;
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
        log_ansi!(0, "\x1b[1;97m", "Resetting emulated system");
        self.state.as_mut().unwrap().reset();
        log!(0, "Reset complete");
    }

    //TODO these will be valid across launches and stops of the emulation thread, but can be called whenever we're stopped to recreate them if needed
    pub fn get_render_reciever(self: &mut Self) -> RenderReciever {
        debug_assert!(!self.thread_running());
        return self.state.as_mut().unwrap().get_render_reciever();
    }

    pub fn get_sound_reciever(self: &mut Self) -> SoundReciever {
        debug_assert!(!self.thread_running());
        return self.state.as_mut().unwrap().get_sound_reciever();
    }

    pub fn get_input_sender(self: &mut Self) -> InputSender {
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
        //debug_assert!(state_for_thread.ready());//TODO

        //Launch the thread
        self.emulation_thread_join_handle.replace(
            thread::Builder::new()
                .name("VSEMUR emulation thread".to_string())
                .spawn(
                    move || -> State {
                        return emulation_thread(state_for_thread, rx);
                    }
                )
                .expect("Failed to launch VSEMUR emulation thread")
        );
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
            panic!("VSEMUR emulation thread dropped reciever (likely due to panic)");
        }

        let old_join_handle = self.emulation_thread_join_handle.take().unwrap();
        let state_from_thread = old_join_handle.join().expect("VSEMUR emulation thread panicked");

        drop(moved_stop_request_sender);

        //Replace the state in our Emulator struct so we can restart it again later
        self.state.replace(state_from_thread);

        log_ansi!(0, "\x1b[1;97m", "Emulation thread stopped");
    }
}

/* Functions */

fn emulation_thread(mut state: State, stop_request_reciever: Receiver<()>) -> State {
    log_ansi!(0, "\x1b[1;97m", "Emulation thread started");

    //The frame loop
    let mut frame_counter: u128 = 0;//Only used in debug build logs
    loop {
        log_ansi!(0, "\x1b[1;94m", "Start of frame {}", frame_counter);
        let start_of_frame = std::time::Instant::now();

        //Check if we've recieved a request to exit, and if so, break out of the loop
        //Checking this once per frame is less expensive than once per emulated clock cycle
        if matches!(stop_request_reciever.try_recv(), Ok(())) {
            log_ansi!(0, "\x1b[1;97m", "Emulation thread stop request recieved");
            break;
        }

        let mut i: usize = 0;
        while i < CYCLES_PER_FRAME {
            log_increment_ticks!();//Increment the number of ticks for debugging
            log_ansi!(0, "\x1b[1;97m", "Cycle block begins");

            //TODO redefine what a "tick" is since it is no longer a clock cycle (perhaps switch to instruction count instead)
            let cycles_executed = unsp::emulate_inst(&mut state);
            log!(1, "Ticking VSmile internal and external peripherals {} time(s) to match", cycles_executed);
            for _ in 0..cycles_executed {
                state.tick();
            }
            //unsp::handle_interrupts(&mut state);//TODO enable this once both this function and the interrupt functionality in State are implemented

            log!(0, "Cycle block ends");

            if state.frame_ended() {//We want to sync the number of ticks we perform with actual frames, not just use frames as a measure of rate-limiting
                break;
            }

            i += cycles_executed as usize;
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

        log_ansi!(0, "\x1b[94m", "End of frame {}", frame_counter);
        frame_counter += 1;
    }

    return state;//Give the state back when we're finished with it
}
