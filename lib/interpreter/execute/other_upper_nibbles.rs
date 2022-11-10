/* NAME//TODO
 * By: John Jekel
 *
 * TODO description
 *
*/

/* Imports */

use crate::logging::*;
use crate::interpreter::State;
use crate::interpreter::Inst;

/* Constants */

//TODO

/* Macros */

//TODO (also pub(crate) use the_macro statements here too)

/* Static Variables */

//TODO

/* Types */

//TODO

/* Associated Functions and Methods */

//TODO

/* Functions */

pub(super) fn execute(state: &mut State, inst: &Inst, upper_nibble: u8, secondary_group: u8) {
    debug_assert!(upper_nibble < 16);
    debug_assert!(secondary_group < 8);
    log!(state.t, 2, "Decoding instruction by secondary group: {:#05b}", secondary_group);

    log_noln!(state.t, 3, "Instruction: ");
    match secondary_group {
        0b000 => { secondary_group_000(state, inst, upper_nibble); },
        0b001 => { secondary_group_001(state, inst, upper_nibble); },
        0b010 => { secondary_group_010(state, inst, upper_nibble); },
        0b011 => { secondary_group_011(state, inst, upper_nibble); },
        0b100 => { secondary_group_100(state, inst, upper_nibble); },
        0b101 => { secondary_group_101(state, inst, upper_nibble); },
        0b110 => { secondary_group_110(state, inst, upper_nibble); },
        0b111 => { secondary_group_111(state, inst, upper_nibble); },
        _ => { if cfg!(debug_assertions) { panic!(); }},//This should never occur
    }
}

fn secondary_group_000(state: &mut State, inst: &Inst, upper_nibble: u8) {
    unimplemented!();
}

fn secondary_group_001(state: &mut State, inst: &Inst, upper_nibble: u8) {
    unimplemented!();
}

fn secondary_group_010(state: &mut State, inst: &Inst, upper_nibble: u8) {
    unimplemented!();
}

fn secondary_group_011(state: &mut State, inst: &Inst, upper_nibble: u8) {
    unimplemented!();
}

fn secondary_group_100(state: &mut State, inst: &Inst, upper_nibble: u8) {
    unimplemented!();
}

fn secondary_group_101(state: &mut State, inst: &Inst, upper_nibble: u8) {
    unimplemented!();
}

fn secondary_group_110(state: &mut State, inst: &Inst, upper_nibble: u8) {
    unimplemented!();
}

fn secondary_group_111(state: &mut State, inst: &Inst, upper_nibble: u8) {
    unimplemented!();
}
