/* NAME//TODO
 * By: John Jekel
 *
 * TODO description
 *
*/

/* Imports */

use crate::logging::*;
use crate::interpreter::memory::MemoryState;
use super::CPUState;

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

pub(super) fn execute(t: u128, cpu: &mut CPUState, mem: &mut MemoryState, inst_word: u16) {
    let secondary_group = (inst_word >> 6) & 0b111;
    debug_assert!(secondary_group < 8);

    log_noln!(t, 4, "Instruction type: ");
    match secondary_group {
        0b000 => { secondary_group_000(t, cpu, mem, inst_word); },
        0b001 => { secondary_group_001(t, cpu, mem, inst_word); },
        0b010 => { secondary_group_010(t, cpu, mem, inst_word); },
        0b011 => { secondary_group_011(t, cpu, mem, inst_word); },
        0b100 => { secondary_group_100(t, cpu, mem, inst_word); },
        0b101 => { secondary_group_101(t, cpu, mem, inst_word); },
        0b110 => { secondary_group_110(t, cpu, mem, inst_word); },
        0b111 => { secondary_group_111(t, cpu, mem, inst_word); },
        _ => { if cfg!(debug_assertions) { panic!(); }},//This should never occur
    }
}

fn secondary_group_000(t: u128, cpu: &mut CPUState, mem: &mut MemoryState, inst_word: u16) {
    //TODO what about multiply
    //Check if bits 11:9 are all set
    if ((inst_word >> 9) & 0b111) == 0b111 {
        log_finln!("DSI6");
        //DS becomes the lower 6 bits of the instruction word
        let new_ds: u8 = (inst_word & 0b111111) as u8;
        cpu.set_ds(new_ds);
        log_noln!(t, 5, "DS becomes {}", new_ds);
    } else {
        //Look at the bits 5:4 to decide what it is
        match (inst_word >> 4) & 0b11 {
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

    cpu.inc_pc();
}

fn secondary_group_001(t: u128, cpu: &mut CPUState, mem: &mut MemoryState, inst_word: u16) {
    log_finln!("CALL");

    //Determine the new cs and pc
    let new_cs: u8 = (inst_word & 0b111111) as u8;
    let new_pc: u16 = super::get_wg2(cpu, mem);
    log!(t, 5, "Get word group 2: {:#06X} | {:#018b}", new_pc, new_pc);
    log!(t, 6, "New CS page, PC address: {:#04X}_{:04X}", new_cs, new_pc);

    //Increment the current PC before pushing to the stack
    cpu.inc_pc_by(2);
    log!(t, 5, "Inc. the current CS page, PC address to {:#04X}_{:04X}", cpu.get_cs(), cpu.pc);
    log!(t, 5, "Push the current PC {:#06X} to the stack @ SP {:#06X}", cpu.pc, cpu.sp);
    super::push_sp(cpu, mem, cpu.pc);
    log!(t, 5, "Push the current SR {:#06X} to the stack @ SP {:#06X}", cpu.sr, cpu.sp);
    super::push_sp(cpu, mem, cpu.sr);

    cpu.set_cs(new_cs);
    cpu.pc = new_pc;
}

fn secondary_group_010(t: u128, cpu: &mut CPUState, mem: &mut MemoryState, inst_word: u16) {
    unimplemented!();
}

fn secondary_group_011(t: u128, cpu: &mut CPUState, mem: &mut MemoryState, inst_word: u16) {
    unimplemented!();
}

fn secondary_group_100(t: u128, cpu: &mut CPUState, mem: &mut MemoryState, inst_word: u16) {
    unimplemented!();
}

fn secondary_group_101(t: u128, cpu: &mut CPUState, mem: &mut MemoryState, inst_word: u16) {
    //Look at bit 5 first to split the opcode space in twoish
    if ((inst_word >> 5) & 0b1) == 0b0 {
        //Look at bits 4:2 to split things further
        match (inst_word >> 3) & 0b11 {
            0b000 => {
                log_finln!("INT SET");
                log!(t, 5, "Low bits: {:#04b}", inst_word & 0b11);
                log_noln!(t, 6, "Instruction: INT ");

                //Check the IRQ bit
                if (inst_word & 0b1) == 0b1 {
                    cpu.irq_enabled = true;
                    log_finln!("IRQ");
                } else {
                    cpu.irq_enabled = false;
                }

                //Check the FIQ bit
                if ((inst_word >> 1) & 0b1) == 0b1 {
                    cpu.fiq_enabled = true;
                    log_finln!("{}FIQ", if cpu.irq_enabled { ", " } else { "" });
                } else {
                    cpu.fiq_enabled = false;
                }

                if cfg!(debug_assertions) && !cpu.irq_enabled && !cpu.fiq_enabled {
                    log_finln!("OFF");
                }

                //Next instruction
                cpu.inc_pc();
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
        match inst_word & 0b111 {
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
                cpu.inc_pc();
            },
            0b011 => {
                log_finln!("DIVQ");
                unimplemented!();//TODO
                cpu.inc_pc();
            },
            0b100 => {
                log_finln!("EXP");
                unimplemented!();//TODO
                cpu.inc_pc();
            },
            0b101 => {
                log_finln!("NOP");
                cpu.inc_pc();//Do nothing, just go to the next instruction
            },
            _ => {//TODO should we do some sort of error handling for this, or do we need to jump somewhere if this occurs?
                log_finln!("(invalid)");
            },
        }
    }
}

fn secondary_group_110(t: u128, cpu: &mut CPUState, mem: &mut MemoryState, inst_word: u16) {
    unimplemented!();
}

fn secondary_group_111(t: u128, cpu: &mut CPUState, mem: &mut MemoryState, inst_word: u16) {
    unimplemented!();
}
