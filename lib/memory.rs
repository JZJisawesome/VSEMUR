use crate::logging::log;
use crate::types::State;
use crate::types::Inst;

pub(crate) fn fetch(state: &State) -> Inst {

    let pc_byte: u16 = 12345;//TODO
    log!(state.num_ticks, 1, "Fetch started from PC={:#x}", pc_byte);

    return Inst{wg: [0, 0]};//TODO
}
