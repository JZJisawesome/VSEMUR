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

    log_noln!(state.t, 4, "Instruction type: ");
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
    //TODO refactor this for better code between IMM6 and IMM16 and Direct16/for clairty
    //TODO we can't just fetch from memory willy-nilly; we need to use memory access functions in case we are writing to/reading from registers/etc

    //IMM6 or branches
    let rd_index: u8 = ((inst.wg[0] >> 9) & 0b111) as u8;

    if rd_index == 0b111 {
        log_finln!("Branch");
        unimplemented!();
    } else {
        log_finln!("IMM6");

        //Get imm6
        let imm6: u8 = (inst.wg[0] & 0b111111) as u8;
        log!(state.t, 5, "IMM6:  {:#06X} | {:#018b} | unsigned {}", imm6, imm6, imm6);

        //Get Rs
        let rs_index: u8 = (inst.wg[0] & 0b111) as u8;
        log_noln!(state.t, 5, "Rs: {:#05b}, aka ", rs_index);
        let rs: u16 = get_rs(state, rs_index);
        log_finln!(", which contains:");
        log!(state.t, 6, "     {:#06X} | {:#018b} | unsigned {}", rs, rs, rs);

        //TODO handle store, cmp, jmp, etc
        log_noln!(state.t, 5, "Operation: ");
        let mut result: u16 = alu_operation(upper_nibble, rs, imm6 as u16);//TODO alu operations may need to set flags; will have to pass in the state

        //Set Rd
        let rd_index: u8 = ((inst.wg[0] >> 9) & 0b111) as u8;
        log_noln!(state.t, 5, "Rd: {:#05b}, aka ", rd_index);
        set_rd(state, rd_index, result);
        log_finln!(", has been set to:");
        log!(state.t, 6, "     {:#06X} | {:#018b} | unsigned {}", result, result, result);
    }

    state.regs.pc += 1;
}

fn secondary_group_010(state: &mut State, inst: &Inst, upper_nibble: u8) {
    log_finln!("Stack Operation");

    log_noln!(state.t, 5, "Instruction: ");
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

fn secondary_group_011(state: &mut State, inst: &Inst, upper_nibble: u8) {
    unimplemented!();
}

fn secondary_group_100(state: &mut State, inst: &Inst, upper_nibble: u8) {
    //TODO refactor this for better code between IMM6 and IMM16 and Direct16/for clairty
    //TODO we can't just fetch from memory willy-nilly; we need to use memory access functions in case we are writing to/reading from registers/etc

    //Direct16 stuffs
    let mut direct16: bool = false;
    let mut direct16_w: bool = false;
    let mut direct16_address: usize = 0;

    //Get the second operand based on bits 5:3, and also set the direct16 flags
    let mut operand2: u16;
    match (inst.wg[0] >> 3) & 0b111 {
        0b001 => {
            log_finln!("IMM16");

            //Get the other operand
            operand2 = inst.wg[1];
            log!(state.t, 5, "IMM16: {:#06X} | {:#018b} | unsigned {}", operand2, operand2, operand2);
        },
        0b010 | 0b011 => {
            log_finln!("Direct16");
            direct16 = true;
            direct16_w = ((inst.wg[0] >> 3) & 0b1) == 0b1;

            direct16_address = (inst.wg[1] as usize) | ((state.regs.sr.ds as usize) << 16);
            operand2 = state.mem[direct16_address];
            log!(state.t, 5, "Address: {:#04X}_{:04X}, which contains", direct16_address >> 16, direct16_address & 0xFFFF);
            log!(state.t, 6, "     {:#06X} | {:#018b} | unsigned {}", operand2, operand2, operand2);
        },
        _ => {//TODO should we do some sort of error handling for this, or do we need to jump somewhere if this occurs?
            log_finln!("(invalid)");
            return;
        },
    }

    //Get Rs
    let rs_index: u8 = (inst.wg[0] & 0b111) as u8;
    log_noln!(state.t, 5, "Rs: {:#05b}, aka ", rs_index);
    let rs: u16 = get_rs(state, rs_index);
    log_finln!(", which contains:");
    log!(state.t, 6, "     {:#06X} | {:#018b} | unsigned {}", rs, rs, rs);

    //Perform the operation
    log_noln!(state.t, 5, "Operation: ");
    if upper_nibble == 0b1101 {//STORE needs special handling
        log_finln!("STORE");
        if direct16 {
            state.mem[direct16_address] = rs;//TODO ensure this is the correct behaviour
        } else {
            unimplemented!();//TODO
        }
    } else {//Any other alu operation//TODO CMP and TEST also need special handling
        let mut result: u16 = alu_operation(upper_nibble, rs, operand2);//TODO alu operations may need to set flags; will have to pass in the state

        //Set Rd
        let rd_index: u8 = ((inst.wg[0] >> 9) & 0b111) as u8;
        log_noln!(state.t, 5, "Rd: {:#05b}, aka ", rd_index);
        set_rd(state, rd_index, result);
        log_finln!(", has been set to:");
        log!(state.t, 6, "     {:#06X} | {:#018b} | unsigned {}", result, result, result);
    }

    state.regs.pc += 2;//2 instead of 1 since we must skip over the 16 bit immediate
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

//TODO move to seperate module perhaps?
fn get_rs(state: &mut State, rs: u8) -> u16 {
    debug_assert!(rs < 8);
    match rs {
        0b000 => {
            log_midln!("SP");
            return (state.regs.sp & 0xFFFF) as u16;
        },
        0b001 => {
            log_midln!("R1");
            return state.regs.r[0];
        },
        0b010 => {
            log_midln!("R2");
            return state.regs.r[1];
        },
        0b011 => {
            log_midln!("R3");
            return state.regs.r[2];
        },
        0b100 => {
            log_midln!("R4");
            return state.regs.r[3];
        },
        0b101 => {
            log_midln!("BP");
            return state.regs.bp;
        },
        0b110 => {
            log_midln!("SR");
            unimplemented!();//TODO
        },
        0b111 => {
            log_midln!("PC");
            return (state.regs.pc & 0xFFFF) as u16;
        },
        _ => { if cfg!(debug_assertions) { panic!(); } return 0; },//This should never occur
    }
}
fn set_rd(state: &mut State, rd: u8, value: u16) {
    debug_assert!(rd < 8);
    match rd {
        0b000 => {
            log_midln!("SP");
            state.regs.sp = value;
        },
        0b001 => {
            log_midln!("R1");
            state.regs.r[0] = value;
        },
        0b010 => {
            log_midln!("R2");
            state.regs.r[1] = value;
        },
        0b011 => {
            log_midln!("R3");
            state.regs.r[2] = value;
        },
        0b100 => {
            log_midln!("R4");
            state.regs.r[3] = value;
        },
        0b101 => {
            log_midln!("BP");
            state.regs.bp = value;
        },
        0b110 => {
            log_midln!("SR");
            unimplemented!();//TODO
        },
        0b111 => {
            log_midln!("PC");
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
