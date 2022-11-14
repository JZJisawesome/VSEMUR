/* NAME//TODO
 * By: John Jekel
 *
 * TODO description
 *
*/

//TODO remove this once everything is implemented
#![allow(unused_variables)]
#![allow(dead_code)]

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

pub(super) fn execute(cpu: &mut CPUState, mem: &mut MemoryState, inst_word: u16) {
    log_noln!(4, "Instruction type: ");
    match super::secondary_group!(inst_word) {
        0b000 => { secondary_group_000(cpu, mem, inst_word); },
        0b001 => { secondary_group_001(cpu, mem, inst_word); },
        0b010 => { secondary_group_010(cpu, mem, inst_word); },
        0b011 => { secondary_group_011(cpu, mem, inst_word); },
        0b100 => { secondary_group_100(cpu, mem, inst_word); },
        0b101 => { secondary_group_101(cpu, mem, inst_word); },
        0b110 => { secondary_group_110(cpu, mem, inst_word); },
        0b111 => { secondary_group_111(cpu, mem, inst_word); },
        _ => { panic!(); },//This should never occur
    }
}

fn secondary_group_000(cpu: &mut CPUState, mem: &mut MemoryState, inst_word: u16) {
    //TODO what about multiply
    //Check if bits 11:9 are all set
    if ((inst_word >> 9) & 0b111) == 0b111 {
        log_finln!("DSI6");
        //DS becomes the lower 6 bits of the instruction word
        let new_ds: u8 = (inst_word & 0b111111) as u8;
        cpu.set_ds(new_ds);
        log_noln!(5, "DS becomes {}", new_ds);
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
            _ => {//TODO should we do some sort of error handling for this (TickFail?), or do we need to jump somewhere if this occurs?
                log_finln!("(invalid)");
                panic!();
            },
        }
    }

    cpu.set_cycle_count(2);//All of these take 2 clock cycles
    cpu.inc_pc();
}

fn secondary_group_001(cpu: &mut CPUState, mem: &mut MemoryState, inst_word: u16) {
    log_finln!("CALL A22");

    //Determine the new cs and pc
    let new_cs: u8 = (inst_word & 0b111111) as u8;
    let new_pc: u16 = super::get_wg2(cpu, mem);
    log!(5, "Get word group 2: {:#06X} | {:#018b}", new_pc, new_pc);
    log!(6, "New CS page, PC address: {:#04X}_{:04X}", new_cs, new_pc);

    //Increment the current PC before pushing to the stack
    cpu.inc_pc_by(2);
    log!(5, "Inc. the current CS page, PC address to {:#04X}_{:04X}", cpu.get_cs(), cpu.pc);
    log!(5, "Push the current PC {:#06X} to the stack @ SP {:#06X}", cpu.pc, cpu.sp);
    super::push_sp(cpu, mem, cpu.pc);
    log!(5, "Push the current SR {:#06X} to the stack @ SP {:#06X}", cpu.sr, cpu.sp);
    super::push_sp(cpu, mem, cpu.sr);

    cpu.set_cs(new_cs);
    cpu.pc = new_pc;

    cpu.set_cycle_count(9);
}

fn secondary_group_010(cpu: &mut CPUState, mem: &mut MemoryState, inst_word: u16) {
    unimplemented!();
}

fn secondary_group_011(cpu: &mut CPUState, mem: &mut MemoryState, inst_word: u16) {
    unimplemented!();
}

fn secondary_group_100(cpu: &mut CPUState, mem: &mut MemoryState, inst_word: u16) {
    unimplemented!();
}

fn secondary_group_101(cpu: &mut CPUState, mem: &mut MemoryState, inst_word: u16) {
    //Look at bit 5 first to split the opcode space in twoish
    if ((inst_word >> 5) & 0b1) == 0b0 {
        //Look at bits 4:2 to split things further
        match (inst_word >> 3) & 0b11 {
            0b000 => {
                log_finln!("INT SET");
                log!(5, "Low bits: {:#04b}", inst_word & 0b11);
                log_noln!(6, "Instruction: INT ");

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

                cpu.set_cycle_count(2);
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
            _ => {//TODO should we do some sort of error handling for this (TickFail?), or do we need to jump somewhere if this occurs?
                log_finln!("(invalid)");
                panic!();
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
                cpu.inc_pc();
                unimplemented!();//TODO
            },
            0b011 => {
                log_finln!("DIVQ");
                cpu.inc_pc();
                unimplemented!();//TODO
            },
            0b100 => {
                log_finln!("EXP");
                cpu.inc_pc();
                unimplemented!();//TODO
            },
            0b101 => {
                log_finln!("NOP");
                cpu.set_cycle_count(2);
                cpu.inc_pc();//Do nothing, just go to the next instruction
            },
            _ => {//TODO should we do some sort of error handling for this (TickFail?), or do we need to jump somewhere if this occurs?
                log_finln!("(invalid)");
                panic!();
            },
        }
    }
}

fn secondary_group_110(cpu: &mut CPUState, mem: &mut MemoryState, inst_word: u16) {
    unimplemented!();
}

fn secondary_group_111(cpu: &mut CPUState, mem: &mut MemoryState, inst_word: u16) {
    unimplemented!();
}
