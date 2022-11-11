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

pub(super) fn execute(state: &mut State, inst: &Inst, secondary_group: u8) {
    debug_assert!(secondary_group < 8);

    log_noln!(state.t, 4, "Instruction type: ");
    match secondary_group {
        0b000 => { secondary_group_000(state, inst); },
        0b001 => { secondary_group_001(state, inst); },
        0b010 => { secondary_group_010(state, inst); },
        0b011 => { secondary_group_011(state, inst); },
        0b100 => { secondary_group_100(state, inst); },
        0b101 => { secondary_group_101(state, inst); },
        0b110 => { secondary_group_110(state, inst); },
        0b111 => { secondary_group_111(state, inst); },
        _ => { if cfg!(debug_assertions) { panic!(); }},//This should never occur
    }
}

fn secondary_group_000(state: &mut State, inst: &Inst) {
    //Check if bits 11:9 are all set
    if ((inst.wg[0] >> 9) & 0b111) == 0b111 {
        log_finln!("DSI6");
        //DS becomes the lower 6 bits of the 0th wordgroup
        let new_ds: u8 = (inst.wg[0] & 0b111111) as u8;
        state.regs.sr.ds = new_ds;
        log_noln!(state.t, 5, "DS becomes {}", new_ds);
    } else {
        //Look at the bits 5:4 to decide what it is
        match (inst.wg[0] >> 4) & 0b11 {
            0b10 => {
                log_finln!("DS access");
                unimplemented!();//TODO
            },
            0b11 => {
                log_finln!("FR access");
                unimplemented!();//TODO
            },
            _ => {//TODO should we do some sort of error handling for this, or do we need to jump somewhere if this occurs?
                log_finln!("(invalid)");
            },
        }
    }

    state.regs.pc += 1;
}

fn secondary_group_001(state: &mut State, inst: &Inst) {
    log_finln!("CALL");
    let new_cs: u8 = (inst.wg[0] & 0b111111) as u8;
    let new_pc: u16 = inst.wg[1];
    log!(state.t, 5, "CS page, PC address: {:#04X}_{:04X}", new_cs, new_pc);

    //TODO must also push onto the stack PC and SR before the next step
    unimplemented!();

    state.regs.sr.cs = new_cs;
    state.regs.pc = new_pc;
}

fn secondary_group_010(state: &mut State, inst: &Inst) {
    unimplemented!();
}

fn secondary_group_011(state: &mut State, inst: &Inst) {
    unimplemented!();
}

fn secondary_group_100(state: &mut State, inst: &Inst) {
    unimplemented!();
}

fn secondary_group_101(state: &mut State, inst: &Inst) {
    //Look at bit 5 first to split the opcode space in twoish
    if ((inst.wg[0] >> 5) & 0b1) == 0b0 {
        //Look at bits 4:2 to split things further
        match (inst.wg[0] >> 3) & 0b11 {
            0b000 => {
                log_finln!("INT SET");
                log!(state.t, 5, "Low bits: {:#04b}", inst.wg[0] & 0b11);
                log_noln!(state.t, 6, "Instruction: INT ");

                //Check the IRQ bit
                if (inst.wg[0] & 0b1) == 0b1 {
                    state.irq_enabled = true;
                    log_finln!("IRQ");
                } else {
                    state.irq_enabled = false;
                }

                //Check the FIQ bit
                if ((inst.wg[0] >> 1) & 0b1) == 0b1 {
                    state.fiq_enabled = true;
                    log_finln!("{}FIQ", if state.irq_enabled { ", " } else { "" });
                } else {
                    state.fiq_enabled = false;
                }

                if cfg!(debug_assertions) && !state.irq_enabled && !state.fiq_enabled {
                    log_finln!("OFF");
                }

                //Next instruction
                state.regs.pc += 1;
            },
            0b001 => {
                unimplemented!();
                //TODO
            },
            0b010 => {
                unimplemented!();//TODO
            },
            0b011 => {
                unimplemented!();//TODO
            },
            _ => {//TODO should we do some sort of error handling for this, or do we need to jump somewhere if this occurs?
                log_finln!("(invalid)");
            },
        }
    } else {
        //Look at the lowest 3 bits to decide what it is
        match inst.wg[0] & 0b111 {
            0b000 => {
                log_finln!("BREAK");
                unimplemented!();//TODO
            },
            0b001 => {
                log_finln!("CALLR");
                unimplemented!();//TODO
            },
            0b010 => {
                log_finln!("DIVS");
                unimplemented!();//TODO
                state.regs.pc += 1;
            },
            0b011 => {
                log_finln!("DIVQ");
                unimplemented!();//TODO
                state.regs.pc += 1;
            },
            0b100 => {
                log_finln!("EXP");
                unimplemented!();//TODO
                state.regs.pc += 1;
            },
            0b101 => {
                log_finln!("NOP");
                state.regs.pc += 1;//Do nothing, just go to the next instruction
            },
            _ => {//TODO should we do some sort of error handling for this, or do we need to jump somewhere if this occurs?
                log_finln!("(invalid)");
            },
        }
    }
}

fn secondary_group_110(state: &mut State, inst: &Inst) {
    unimplemented!();
}

fn secondary_group_111(state: &mut State, inst: &Inst) {
    unimplemented!();
}
