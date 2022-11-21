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

pub use state::State;

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

/* Associated Functions and Methods */

/* Functions */

//TODO
