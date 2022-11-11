/* NAME//TODO
 * By: John Jekel
 *
 * TODO description
 *
*/

/* Imports */

use crate::logging::log;

/* Constants */

//TODO

/* Macros */

//TODO (also pub(crate) use the_macro statements here too)

/* Static Variables */

//TODO

/* Types */

pub(super) struct CPUState {
    regs: Registers,
    irq_enabled: bool,
    fiq_enabled: bool,
}

struct Registers {
    sp: u16,
    r: [u16;4],
    bp: u16,
    sr: SR,
    pc: u16,
}

struct SR {//16 bits in all
    //6 bits each
    ds: u8,
    cs: u8,

    //1 bit each
    n: bool,
    z: bool,
    s: bool,
    c: bool,
}

/* Associated Functions and Methods */

impl CPUState {
    pub(super) fn new() -> CPUState {
        unimplemented!();
    }

    pub(super) fn reset(self: &mut Self) {
        log!(0, 1, "Resetting CPU");
        unimplemented!();//TODO
    }
}

/* Functions */

//TODO
