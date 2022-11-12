/* NAME//TODO
 * By: John Jekel
 *
 * TODO description
 *
*/

//TODO remove this once everything is implemented
#![allow(unused_variables)]
#![allow(dead_code)]
#![allow(unreachable_code)]

/* Imports */

use crate::logging::*;
use crate::interpreter::memory::MemoryState;
use super::CPUState;

/* Constants */

//TODO

/* Macros */

//TODO perhaps move these to a common location, like execute.rs?
macro_rules! reg_string_by_index {
    ($rs:expr) => {{
        debug_assert!($rs < 8);
        let string: &str;
        match $rs  {
            0b000 => { string = "SP"; },
            0b001 => { string = "R1"; },
            0b010 => { string = "R2"; },
            0b011 => { string = "R3"; },
            0b100 => { string = "R4"; },
            0b101 => { string = "BP"; },
            0b110 => { string = "SR"; },
            0b111 => { string = "PC"; },
            _ => { if cfg!(debug_assertions) { panic!(); } string = ""; },//This should never occur
        }
        string
    }};
}

macro_rules! log_register {
    ($tick_num:expr, $indent:expr, $reg_name:expr, $reg_index:expr) => {
        log!($tick_num, $indent, "{} is {:#05b}, aka {}", $reg_name, $reg_index, reg_string_by_index!($reg_index));
    };
    ($tick_num:expr, $indent:expr, $reg_name:expr, $reg_index:expr, $reg_contents:expr) => {
        log!($tick_num, $indent, "{} is {:#05b}, aka {}, which contains:", $reg_name, $reg_index, reg_string_by_index!($reg_index));
        log!($tick_num, $indent + 1, "{:#06X} | {:#018b} | unsigned {}", $reg_contents, $reg_contents, $reg_contents);
    };
}

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

    //IMM6 or branches
    let rd_index: u8 = ((inst_word >> 9) & 0b111) as u8;
    if rd_index == 0b111 {
        log_finln!("Branch");
        unimplemented!();
    } else {
        log_finln!("IMM6");

        //Get Rd
        let rd: u16 = get_reg_by_index(cpu, rd_index);
        log_register!(t, 5, "Rd", rd_index, rd);

        //Get imm6
        let imm6: u8 = (inst_word & 0b111111) as u8;
        log!(t, 5, "IMM6:  {:#06X} | {:#018b} | unsigned {}", imm6, imm6, imm6);

        //Perform the operation
        let result: u16 = alu_operation(t, cpu, upper_nibble as u8, rd, imm6 as u16);

        //Write to the appropriate (if any) destination
        match upper_nibble {
            0b0100 | 0b1100 => {},//CMP and TEST write to flags like other instructions, but not to Rd/to memory
            0b1101 => {//IMM6 STORE is invalid (we can't store to an immediate)
                log!(t, 5, "This isn't valid: we can't store a result to an immediate!");
            },
            _ => {//Other cases are much simpler; we just write to Rd
                set_reg_by_index(cpu, rd_index, result);
                log_register!(t, 5, "Rd", rd_index, result);
            }
        }

        cpu.inc_pc();
    }
}

fn secondary_group_010(t: u128, cpu: &mut CPUState, mem: &mut MemoryState, inst_word: u16) {
    let upper_nibble = inst_word >> 12;
    log_finln!("Stack Operation");

    //Get Rs since we index by it
    let rs_index: u8 = (inst_word & 0b111) as u8;
    let mut rs: u16 = get_reg_by_index(cpu, rs_index);
    log_register!(t, 5, "Rs", rs_index, rs);

    //Get Rh
    let mut rh_index: u8 = ((inst_word >> 9) & 0b111) as u8;
    log_register!(t, 5, "Rh", rh_index);
    get_reg_by_index(cpu, rh_index);

    //Get Size
    let mut size: u8 = ((inst_word >> 3) & 0b111) as u8;
    log!(t, 5, "Size is {}", size);

    log_noln!(t, 5, "Instruction: ");
    match upper_nibble {
        0b1101 => {
            //HACK We assume the SP will always point to page 0 (where memory is on the vsmile), so we never update the ds register here for speed
            log_finln!("PUSH");
            while size != 0 {
                //TODO is this the correct order to push things?
                let reg: u16 = get_reg_by_index(cpu, rh_index);
                log_register!(t, 5, "Current reg.", rh_index, reg);
                rs = get_reg_by_index(cpu, rs_index);

                mem.write_page_addr(reg, 0x00, rs);
                log!(t, 7, "Pushed to the stack @ [Rs]: {:#06X}", rs);

                rs -= 1;
                log!(t, 7, "Decrement Rs; it is now {:#06X}", rs);

                rh_index -= 1;

                size -= 1;
            }

            set_reg_by_index(cpu, rs_index, rs);//Actually write back RS to the cpu's state
        },
        0b1001 => {
            //HACK We assume the SP will always point to page 0 (where memory is on the vsmile), so we never update the ds register here for speed
            log_finln!("POP");
            unimplemented!();//TODO figure out the exact semantics of this
        },
        _ => {//TODO should we do some sort of error handling for this, or do we need to jump somewhere if this occurs?
            log_finln!("(invalid)");
        },
    }

    cpu.inc_pc();
}

fn secondary_group_011(t: u128, cpu: &mut CPUState, mem: &mut MemoryState, inst_word: u16) {
    let upper_nibble = inst_word >> 12;
    log_finln!("DS_Indirect");

    //Check D flag and determine page
    let page: u8;
    let d_flag: bool = (inst_word >> 5 & 0b1) == 0b1;
    log_noln!(t, 5, "D flag ");
    if d_flag {
        page = cpu.get_ds();
    } else {
        page = 0x00;
        log_midln!("not ");
    }
    log_finln!("set, page is {:#04X}", page);

    //Get Rd
    let rd_index: u8 = ((inst_word) >> 9 & 0b111) as u8;
    let mut rd: u16 = get_reg_by_index(cpu, rd_index);
    log_register!(t, 5, "Rd", rd_index, rd);

    //Get Rs since we index by it
    let rs_index: u8 = (inst_word & 0b111) as u8;
    let mut rs: u16 = get_reg_by_index(cpu, rs_index);
    log_register!(t, 5, "Rs", rs_index, rs);

    //Do pre-operations to rs
    match (inst_word >> 3) & 0b11 {
        0b00 => {
            log!(t, 5, "@ is 0b00, no change to Rs");
        },
        0b01 | 0b10 => {},//0b01 and 0b10 are post-decrement and increment respectively, so we do nothing here
        0b11 => {
            log!(t, 5, "@ is 0b11, do ++Rs");
            if d_flag {
                let new_ds_rs_tuple = super::super::inc_page_addr_by(page, rs, 1);
                cpu.set_ds(new_ds_rs_tuple.0);
                rs = new_ds_rs_tuple.1;
            } else {
                rs += 1;
            }

            set_reg_by_index(cpu, rs_index, rs);
            //TODO log DS too?
            log_register!(t, 5, "Rs", rs_index, rs);
        },
        _ => { if cfg!(debug_assertions) { panic!(); } },//This should never occur
    }

    //Get data at address determined by page and Rs
    let data: u16 = mem.read_page_addr(page, rs);
    log!(t, 5, "DS page, immediate addr: {:#04X}_{:04X}, which contains", page, rs);
    log!(t, 6, "{:#06X} | {:#018b} | unsigned {}", data, data, data);

    //Perform the operation
    let result: u16 = alu_operation(t, cpu, upper_nibble as u8, rd, data);

    //Store back to Rd
    set_reg_by_index(cpu, rd_index, result);
    log_register!(t, 5, "Rd", rd_index, result);

    //Do post-operations to rs
    match (inst_word >> 3) & 0b11 {
        0b00 | 0b11 => {},//0b00 is no increment/decrement, 0b11 is pre-increment, so we do nothing here
        0b01 => {
            log!(t, 5, "@ is 0b01, do Rs--");
            unimplemented!();//TODO may also have to modify pages if d flag is set
            set_reg_by_index(cpu, rs_index, rs);
        },
        0b10 => {
            log!(t, 5, "@ is 0b10, do Rs++");
            if d_flag {
                let new_ds_rs_tuple = super::super::inc_page_addr_by(page, rs, 1);
                cpu.set_ds(new_ds_rs_tuple.0);
                rs = new_ds_rs_tuple.1;
            } else {
                rs += 1;
            }

            set_reg_by_index(cpu, rs_index, rs);
            //TODO log DS too?
            log_register!(t, 5, "Rs", rs_index, rs);
        },
        _ => { if cfg!(debug_assertions) { panic!(); } },//This should never occur
    }

    cpu.inc_pc();
}

fn secondary_group_100(t: u128, cpu: &mut CPUState, mem: &mut MemoryState, inst_word: u16) {
    let upper_nibble = inst_word >> 12;
    //Flags to decide output behaviour (upper_nibble is also used)
    let direct16: bool;
    let direct16_w: bool;

    //Operands
    let operand1: u16;
    let operand2: u16;

    //Determine if this is IMM16 or Direct16 and perform type-specific setup
    match ((inst_word >> 4) & 0b11, (inst_word >> 3) & 0b1) {
        (0b00, 0b1) => {
            log_finln!("IMM16");
            direct16 = false;
            direct16_w = false;//This value dosn't matter

            //Get the operands
            //Rs is operand1
            let rs_index: u8 = (inst_word & 0b111) as u8;
            operand1 = get_reg_by_index(cpu, rs_index);
            log_register!(t, 5, "Rs", rs_index, operand1);

            //IMM16 is operand2
            operand2 = super::get_wg2(cpu, mem);
            log!(t, 5, "IMM16: {:#06X} | {:#018b} | unsigned {}", operand2, operand2, operand2);
        },
        (0b01, direct16_w_bit) => {
            log_midln!("Direct16");
            direct16 = true;
            direct16_w = direct16_w_bit == 0b1;
            log_finln!(", with W flag{} set", if direct16_w { "" } else { " not" });

            //Get the operands
            //Rs is always one of the operands
            let rs_index: u8 = (inst_word & 0b111) as u8;
            let rs = get_reg_by_index(cpu, rs_index);
            log_register!(t, 5, "Rs", rs_index, rs);

            if direct16_w {
                //Rd is operand1
                let rd_index: u8 = ((inst_word >> 9) & 0b111) as u8;
                operand1 = get_reg_by_index(cpu, rd_index);
                log_register!(t, 5, "Rd", rd_index, operand1);

                //Rs is operand2
                operand2 = rs;
            } else {
                //Rs is operand1
                operand1 = rs;

                //The word at the memory address is operand2
                let page = cpu.get_ds();
                let addr = super::get_wg2(cpu, mem);
                operand2 = mem.read_page_addr(page, addr);
                log!(t, 5, "DS page, immediate addr: {:#04X}_{:04X}, which contains", page, addr);
                log!(t, 6, "     {:#06X} | {:#018b} | unsigned {}", operand2, operand2, operand2);
            }
        },
        (_, _) => {//TODO should we do some sort of error handling for this, or do we need to jump somewhere if this occurs?
            log_finln!("(invalid)");
            return;
        },
    }

    //Perform the operation
    let result: u16 = alu_operation(t, cpu, upper_nibble as u8, operand1, operand2);

    //Write to the appropriate (if any) destination
    match (upper_nibble, direct16, direct16_w) {
        (0b0100, _, _) | (0b1100, _, _) => {},//CMP and TEST write to flags like other instructions, but not to Rd/to memory
        (0b1101, false, _) => {//IMM16 STORE is invalid (we can't store to an immediate)
            log!(t, 5, "This isn't valid: we can't store a result to an immediate!");
        },
        (0b1101, true, true) => {//Direct16 STORE + w flag set stores the result (which is Rd) to Rs
            let rs_index: u8 = (inst_word & 0b111) as u8;
            set_reg_by_index(cpu, rs_index, result);//rs is rd, and rd is result
            log_register!(t, 5, "Rs", rs_index, result);
        },
        (0b1101, true, false) |//Direct16 STORE + w flag not set stores the result (which is Rs) to memory
        (_, true, true) => {//Direct16 operation with w flag set writes result to memory instead of a register
            unimplemented!();//TODO
        }
        (_, false, _) | (_, true, false) => {//Other cases are much simpler; we just write to Rd
            let rd_index: u8 = ((inst_word >> 9) & 0b111) as u8;
            set_reg_by_index(cpu, rd_index, result);
            log_register!(t, 5, "Rd", rd_index, result);
        }
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

//TODO move to seperate module perhaps (up to execute.rs)?
fn get_reg_by_index(cpu: &CPUState, rs: u8) -> u16 {
    debug_assert!(rs < 8);
    match rs {
        0b000 => {
            return (cpu.sp & 0xFFFF) as u16;
        },
        0b001 => {
            return cpu.r[0];
        },
        0b010 => {
            return cpu.r[1];
        },
        0b011 => {
            return cpu.r[2];
        },
        0b100 => {
            return cpu.r[3];
        },
        0b101 => {
            return cpu.bp;
        },
        0b110 => {
            return cpu.sr;
        },
        0b111 => {
            return cpu.pc;
        },
        _ => { if cfg!(debug_assertions) { panic!(); } return 0; },//This should never occur
    }
}
fn set_reg_by_index(cpu: &mut CPUState, rd: u8, value: u16) {
    debug_assert!(rd < 8);
    match rd {
        0b000 => {
            cpu.sp = value;
        },
        0b001 => {
            cpu.r[0] = value;
        },
        0b010 => {
            cpu.r[1] = value;
        },
        0b011 => {
            cpu.r[2] = value;
        },
        0b100 => {
            cpu.r[3] = value;
        },
        0b101 => {
            cpu.bp = value;
        },
        0b110 => {
            cpu.sr = value;
        },
        0b111 => {
            cpu.pc = value;
            unimplemented!();//TODO what are the implications of the increment after this?
        },
        _ => { if cfg!(debug_assertions) { panic!(); } },//This should never occur
    }
}
fn alu_operation(t: u128, cpu: &mut CPUState, upper_nibble: u8, operand1: u16, operand2: u16) -> u16 {//Needs mutable reference to CPUState to sets flags properly
    use std::num::Wrapping as Wrap;

    //We need regular wrapping behaviour to make our lives easier; also do 32 bit operations so we get the carry bit (which is useful) for free
    let operand1_w = Wrap(operand1 as u32);
    let operand2_w = Wrap(operand2 as u32);

    //Perform operation
    log_noln!(t, 5, "Operation: ");
    let result_w: Wrap<u32>;
    match upper_nibble {
        0b0000 => {
            log_finln!("ADD");
            result_w = operand1_w + operand2_w;
        },
        0b0001 => {
            log_finln!("ADC");
            result_w = operand1_w + operand2_w + if cpu.get_c() { Wrap(1) } else { Wrap(0) };
        },
        0b0010 => {
            log_finln!("SUB");
            result_w = operand1_w - operand2_w;
        },
        0b0011 => {
            log_finln!("SBC");
            result_w = operand1_w + !operand2_w + if cpu.get_c() { Wrap(1) } else { Wrap(0) };
        },
        0b0100 => {
            log_finln!("CMP");
            result_w = operand1_w - operand2_w;
        },
        0b0110 => {
            log_finln!("NEG");
            result_w = Wrap((-(operand2 as i32)) as u32);//Intentionally not using operand2_w so that we can cast to a signed integer and back//TODO ensure this is valid, else do ~operand2 + 1
        },
        0b1000 => {
            log_finln!("XOR");
            result_w = operand1_w ^ operand2_w;
        },
        0b1001 => {
            log_finln!("LOAD");
            result_w = operand2_w;
        },
        0b1010 => {
            log_finln!("OR");
            result_w = operand1_w | operand2_w;
        },
        0b1011 => {
            log_finln!("AND");
            result_w = operand1_w & operand2_w;
        },
        0b1100 => {
            log_finln!("TEST");
            result_w = operand1_w & operand2_w;
        },
        0b1101 => {
            log_finln!("STORE");
            result_w = operand1_w;//No need for any flags to be set with store
        },
        _ => {//TODO should we do some sort of error handling for this, or do we need to jump somewhere if this occurs?
            log_finln!("(invalid)");
            return 0;
        },
    }
    let result: u32 = result_w.0;//We don't need wrapping behaviour anymore
    log!(t, 5, "Result:{:#06X} | {:#018b} | unsigned {}", (result & 0xFFFF) as u16, (result & 0xFFFF) as u16, (result & 0xFFFF) as u16);

    //Set flags
    //FIXME don't update flags if the register is the PC
    //N flag is set if the result's msb is 1
    //Z flag is set if the result is 0
    //S flag is set if the result is negative (not the same as N since it looks at higher bits too)
    //C flag is set if there was a carry
    match upper_nibble {
        0b0000 | 0b0001 | 0b0010 | 0b0011 | 0b0100 => {//ADD, ADC, SUB, SBC, CMP update all flags
            cpu.set_n(((result >> 15) & 0b1) == 0b1);
            cpu.set_z(result == 0);
            cpu.set_s((result as i32) < 0);//TODO ensure this is correct; mame does this differently
            cpu.set_c(((result >> 16) & 0b1) == 0b1);
        },
        0b0110 | 0b1000 | 0b1001 | 0b1010 | 0b1011 | 0b1100 => {//NEG, XOR, LOAD, OR, AND, TEST update only N, Z flags
            cpu.set_n(((result >> 15) & 0b1) == 0b1);
            cpu.set_z(result == 0);
        },
        0b1101 => {},//STORE dosn't update flags
        _ => { return 0; },//TODO should we do some sort of error handling for this, or do we need to jump somewhere if this occurs?
    }


    return (result & 0xFFFF) as u16;
}
