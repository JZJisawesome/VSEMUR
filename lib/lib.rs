pub mod about;//We want to export the module itself, into the root namespace, not it's contents directly, so we don't use ::*
mod execute;
mod logging;
mod memory;
mod types;

pub use types::State;
pub use types::ReturnCode;




//TODO organize the below better


use logging::log;
pub fn tick(state: &mut State) -> ReturnCode {
    state.num_ticks += 1;
    log!(state.num_ticks, 0, "Tick {} begins", state.num_ticks);
    crate::memory::fetch(&state);
    return ReturnCode::FAIL;
}

