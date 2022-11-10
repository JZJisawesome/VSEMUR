use crate::logging::log;
use crate::interpreter::State;
use crate::interpreter::Inst;

pub(super) fn fetch(state: &State) -> Inst {

    let pc_byte: u16 = 245;//TODO
    log!(state.t, 1, "Fetch started from PC={:#06X}", pc_byte);

    return Inst{wg: [0, 0]};//TODO
}
