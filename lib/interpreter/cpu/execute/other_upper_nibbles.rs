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
    let upper_nibble = inst_word >> 12;
    let secondary_group = (inst_word >> 6) & 0b111;
    debug_assert!(upper_nibble < 16);
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
    unimplemented!();
}

fn secondary_group_001(t: u128, cpu: &mut CPUState, mem: &mut MemoryState, inst_word: u16) {
    let upper_nibble = inst_word >> 12;
    //TODO refactor this for better code between IMM6 and IMM16 and Direct16/for clairty
    //TODO we can't just fetch from memory willy-nilly; we need to use memory access functions in case we are writing to/reading from registers/etc

    //IMM6 or branches
    let rd_index: u8 = ((inst_word >> 9) & 0b111) as u8;

    if rd_index == 0b111 {
        log_finln!("Branch");
        unimplemented!();
    } else {
        log_finln!("IMM6");

        //Get imm6
        let imm6: u8 = (inst_word & 0b111111) as u8;
        log!(t, 5, "IMM6:  {:#06X} | {:#018b} | unsigned {}", imm6, imm6, imm6);

        //Get Rs
        let rs_index: u8 = (inst_word & 0b111) as u8;
        log_noln!(t, 5, "Rs: {:#05b}, aka ", rs_index);
        let rs: u16 = get_rs(cpu, rs_index);
        log_finln!(", which contains:");
        log!(t, 6, "     {:#06X} | {:#018b} | unsigned {}", rs, rs, rs);

        //TODO handle store, cmp, jmp, etc
        log_noln!(t, 5, "Operation: ");
        let mut result: u16 = alu_operation(upper_nibble as u8, rs, imm6 as u16);//TODO alu operations may need to set flags; will have to pass in the state

        //Set Rd
        let rd_index: u8 = ((inst_word >> 9) & 0b111) as u8;
        log_noln!(t, 5, "Rd: {:#05b}, aka ", rd_index);
        set_rd(cpu, rd_index, result);
        log_finln!(", has been set to:");
        log!(t, 6, "     {:#06X} | {:#018b} | unsigned {}", result, result, result);
    }

    cpu.inc_pc();
}

fn secondary_group_010(t: u128, cpu: &mut CPUState, mem: &mut MemoryState, inst_word: u16) {
    let upper_nibble = inst_word >> 12;
    log_finln!("Stack Operation");

    log_noln!(t, 5, "Instruction: ");
    match upper_nibble {
        0b1101 => {
            log_finln!("PUSH");
            unimplemented!();
        },
        0b1001 => {
            log_finln!("POP");
            unimplemented!();
        },
        _ => {//TODO should we do some sort of error handling for this, or do we need to jump somewhere if this occurs?
            log_finln!("(invalid)");
        },
    }
}

fn secondary_group_011(t: u128, cpu: &mut CPUState, mem: &mut MemoryState, inst_word: u16) {
    unimplemented!();
}

fn secondary_group_100(t: u128, cpu: &mut CPUState, mem: &mut MemoryState, inst_word: u16) {
    let upper_nibble = inst_word >> 12;
    //TODO refactor this for better code between IMM6 and IMM16 and Direct16/for clairty
    //TODO we can't just fetch from memory willy-nilly; we need to use memory access functions in case we are writing to/reading from registers/etc

    //Direct16 stuffs
    let mut direct16: bool = false;
    let mut direct16_w: bool = false;
    let mut direct16_address: usize = 0;

    //Get the second operand based on bits 5:3, and also set the direct16 flags
    let mut operand2: u16;
    match (inst_word >> 3) & 0b111 {
        0b001 => {
            log_finln!("IMM16");

            //Get the other operand
            operand2 = super::get_wg1(cpu, mem);
            log!(t, 5, "IMM16: {:#06X} | {:#018b} | unsigned {}", operand2, operand2, operand2);
        },
        0b010 | 0b011 => {
            log_finln!("Direct16");
            direct16 = true;
            direct16_w = ((inst_word >> 3) & 0b1) == 0b1;

            direct16_address = (super::get_wg1(cpu, mem) as usize) | ((cpu.get_ds() as usize) << 16);
            //operand2 = state.mem[direct16_address];
            unimplemented!();//TODO
            log!(t, 5, "Address: {:#04X}_{:04X}, which contains", direct16_address >> 16, direct16_address & 0xFFFF);
            log!(t, 6, "     {:#06X} | {:#018b} | unsigned {}", operand2, operand2, operand2);
        },
        _ => {//TODO should we do some sort of error handling for this, or do we need to jump somewhere if this occurs?
            log_finln!("(invalid)");
            return;
        },
    }

    //Get Rs
    let rs_index: u8 = (inst_word & 0b111) as u8;
    log_noln!(t, 5, "Rs: {:#05b}, aka ", rs_index);
    let rs: u16 = get_rs(cpu, rs_index);
    log_finln!(", which contains:");
    log!(t, 6, "     {:#06X} | {:#018b} | unsigned {}", rs, rs, rs);

    //Perform the operation
    log_noln!(t, 5, "Operation: ");
    if upper_nibble == 0b1101 {//STORE needs special handling
        log_finln!("STORE");
        if direct16 {
            //state.mem[direct16_address] = rs;//TODO ensure this is the correct behaviour
            unimplemented!();//TODO
        } else {
            unimplemented!();//TODO
        }
    } else {//Any other alu operation//TODO CMP and TEST also need special handling
        let mut result: u16 = alu_operation(upper_nibble as u8, rs, operand2);//TODO alu operations may need to set flags; will have to pass in the state

        //Set Rd
        let rd_index: u8 = ((inst_word >> 9) & 0b111) as u8;
        log_noln!(t, 5, "Rd: {:#05b}, aka ", rd_index);
        set_rd(cpu, rd_index, result);
        log_finln!(", has been set to:");
        log!(t, 6, "     {:#06X} | {:#018b} | unsigned {}", result, result, result);
    }

    cpu.inc_pc_by(2);//2 instead of 1 since we must skip over the 16 bit immediate
}

fn secondary_group_101(t: u128, cpu: &mut CPUState, mem: &mut MemoryState, inst_word: u16) {
    unimplemented!();
}

fn secondary_group_110(t: u128, cpu: &mut CPUState, mem: &mut MemoryState, inst_word: u16) {
    unimplemented!();
}

fn secondary_group_111(t: u128, cpu: &mut CPUState, mem: &mut MemoryState, inst_word: u16) {
    unimplemented!();
}

//TODO move to seperate module perhaps?
fn get_rs(cpu: &CPUState, rs: u8) -> u16 {
    debug_assert!(rs < 8);
    match rs {
        0b000 => {
            log_midln!("SP");
            return (cpu.sp & 0xFFFF) as u16;
        },
        0b001 => {
            log_midln!("R1");
            return cpu.r[0];
        },
        0b010 => {
            log_midln!("R2");
            return cpu.r[1];
        },
        0b011 => {
            log_midln!("R3");
            return cpu.r[2];
        },
        0b100 => {
            log_midln!("R4");
            return cpu.r[3];
        },
        0b101 => {
            log_midln!("BP");
            return cpu.bp;
        },
        0b110 => {
            log_midln!("SR");
            return cpu.sr;
        },
        0b111 => {
            log_midln!("PC");
            return cpu.pc;
        },
        _ => { if cfg!(debug_assertions) { panic!(); } return 0; },//This should never occur
    }
}
fn set_rd(cpu: &mut CPUState, rd: u8, value: u16) {
    debug_assert!(rd < 8);
    match rd {
        0b000 => {
            log_midln!("SP");
            cpu.sp = value;
        },
        0b001 => {
            log_midln!("R1");
            cpu.r[0] = value;
        },
        0b010 => {
            log_midln!("R2");
            cpu.r[1] = value;
        },
        0b011 => {
            log_midln!("R3");
            cpu.r[2] = value;
        },
        0b100 => {
            log_midln!("R4");
            cpu.r[3] = value;
        },
        0b101 => {
            log_midln!("BP");
            cpu.bp = value;
        },
        0b110 => {
            log_midln!("SR");
            cpu.sr = value;
        },
        0b111 => {
            log_midln!("PC");
            cpu.pc = value;
            unimplemented!();//TODO what are the implications of the increment after this?
        },
        _ => { if cfg!(debug_assertions) { panic!(); } },//This should never occur
    }
}
fn alu_operation(upper_nibble: u8, operand1: u16, operand2: u16) -> u16 {
    match upper_nibble {
        0b0000 => {
            log_finln!("ADD");
            return operand1 + operand2;
        },
        0b0001 => {
            log_finln!("ADC");
            unimplemented!();//TODO
        },
        0b0010 => {
            log_finln!("SUB");
            return operand1 - operand2;
        },
        0b0011 => {
            log_finln!("SBC");
            unimplemented!();//TODO
        },
        0b0100 => {
            log_finln!("CMP");
            unimplemented!();//TODO
        },
        0b0110 => {
            log_finln!("NEG");
            return ((-(operand2 as i32)) & 0xFFFF) as u16;//TODO ensure this is valid, else do ~operand2 + 1
        },
        0b1000 => {
            log_finln!("XOR");
            return operand1 ^ operand2;
        },
        0b1001 => {
            log_finln!("LOAD");
            return operand2;
        },
        0b1010 => {
            log_finln!("OR");
            return operand1 | operand2;
        },
        0b1011 => {
            log_finln!("AND");
            return operand1 & operand2;
        },
        0b1100 => {
            log_finln!("TEST");
            unimplemented!();//TODO
        },
        0b1101 => {
            log_finln!("STORE");
            unimplemented!();//TODO
        },
        _ => {//TODO should we do some sort of error handling for this, or do we need to jump somewhere if this occurs?
            log_finln!("(invalid)");
            return 0;
        },
    }
}
