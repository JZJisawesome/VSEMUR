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

    //IMM6 or branches
    let rd_index: u8 = ((inst_word >> 9) & 0b111) as u8;
    if rd_index == 0b111 {
        log_finln!("Branch");
        unimplemented!();
    } else {
        log_finln!("IMM6");

        //Get Rd
        log_noln!(t, 5, "Rd is {:#05b}, aka ", rd_index);
        let rd: u16 = get_reg_by_index(cpu, rd_index);
        log_finln!(", which contains:");
        log!(t, 6, "     {:#06X} | {:#018b} | unsigned {}", rd, rd, rd);

        //Get imm6
        let imm6: u8 = (inst_word & 0b111111) as u8;
        log!(t, 5, "IMM6:  {:#06X} | {:#018b} | unsigned {}", imm6, imm6, imm6);

        //Perform the operation
        log_noln!(t, 5, "Operation: ");
        let result: u16 = alu_operation(cpu, upper_nibble as u8, rd, imm6 as u16);
        log!(t, 5, "Result:{:#06X} | {:#018b} | unsigned {}", result, result, result);

        //Write to the appropriate (if any) destination
        match upper_nibble {
            0b0100 | 0b1100 => {},//CMP and TEST write to flags like other instructions, but not to Rd/to memory
            0b1101 => {//IMM6 STORE is invalid (we can't store to an immediate)
                log!(t, 5, "This isn't valid: we can't store a result to an immediate!");
            },
           _ => {//Other cases are much simpler; we just write to Rd
                log_noln!(t, 5, "Rd is {:#05b}, aka ", rd_index);
                set_reg_by_index(cpu, rd_index, result);
                log_finln!(", and has been set to:");
                log!(t, 6, "     {:#06X} | {:#018b} | unsigned {}", result, result, result);
            }
        }

        cpu.inc_pc();
    }
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
            log_noln!(t, 5, "Rs is {:#05b}, aka ", rs_index);
            operand1 = get_reg_by_index(cpu, rs_index);
            log_finln!(", which contains:");
            log!(t, 6, "     {:#06X} | {:#018b} | unsigned {}", operand1, operand1, operand1);
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
            log_noln!(t, 5, "Rs is {:#05b}, aka ", rs_index);
            let rs = get_reg_by_index(cpu, rs_index);
            log_finln!(", which contains:");
            log!(t, 6, "     {:#06X} | {:#018b} | unsigned {}", rs, rs, rs);

            //TODO this seems to cause the stack pointer to be set incorrectly; figure out the actual proper behaviour here
            /*
            if direct16_w {
                //Rs is operand1
                operand1 = rs;

                //The word at the memory address is operand2
                let page = cpu.get_ds();
                let addr = super::get_wg2(cpu, mem);
                operand2 = mem.read_page_addr(page, addr);
                log!(t, 5, "DS page, immediate addr: {:#04X}_{:04X}, which contains", page, addr);
                log!(t, 6, "     {:#06X} | {:#018b} | unsigned {}", operand2, operand2, operand2);
            } else {
                //Rd is operand1
                let rd_index: u8 = ((inst_word >> 9) & 0b111) as u8;
                log_noln!(t, 5, "Rd is {:#05b}, aka ", rd_index);
                operand1 = get_reg_by_index(cpu, rd_index);
                log_finln!(", which contains:");
                log!(t, 6, "     {:#06X} | {:#018b} | unsigned {}", operand1, operand1, operand1);

                //Rs is operand2
                operand2 = rs;
            }
            */
            //TEMPORARY old implementation (FIXME get rid of this and verify/fix the above)
            operand1 = rs;//operand1 is always RS
            let page = cpu.get_ds();
            let addr = super::get_wg2(cpu, mem);
            operand2 = mem.read_page_addr(page, addr);//operand2 is always the data in memory
            log!(t, 5, "DS page, immediate addr: {:#04X}_{:04X}, which contains", page, addr);
            log!(t, 6, "     {:#06X} | {:#018b} | unsigned {}", operand2, operand2, operand2);
        },
        (_, _) => {//TODO should we do some sort of error handling for this, or do we need to jump somewhere if this occurs?
            log_finln!("(invalid)");
            return;
        },
    }

    //Perform the operation
    log_noln!(t, 5, "Operation: ");
    let result: u16 = alu_operation(cpu, upper_nibble as u8, operand1, operand2);
    log!(t, 5, "Result:{:#06X} | {:#018b} | unsigned {}", result, result, result);

    //Write to the appropriate (if any) destination
    match (upper_nibble, direct16, direct16_w) {
        (0b0100, _, _) | (0b1100, _, _) => {},//CMP and TEST write to flags like other instructions, but not to Rd/to memory
        (0b1101, false, _) => {//IMM16 STORE is invalid (we can't store to an immediate)
            log!(t, 5, "This isn't valid: we can't store a result to an immediate!");
        },
        (0b1101, true, true) => {//Direct16 STORE + w flag set stores the result (which is Rd) to Rs
            let rs_index: u8 = (inst_word & 0b111) as u8;
            log_noln!(t, 5, "Rs is {:#05b}, aka ", rs_index);
            set_reg_by_index(cpu, rs_index, result);//rs is rd, and rd is result
            log_finln!(", and has been set to:");
            log!(t, 6, "     {:#06X} | {:#018b} | unsigned {}", result, result, result);
        },
        (0b1101, true, false) |//Direct16 STORE + w flag not set stores the result (which is Rs) to memory
        (_, true, true) => {//Direct16 operation with w flag set writes result to memory instead of a register
            unimplemented!();//TODO
        }
        (_, false, _) | (_, true, false) => {//Other cases are much simpler; we just write to Rd
            let rd_index: u8 = ((inst_word >> 9) & 0b111) as u8;
            log_noln!(t, 5, "Rd is {:#05b}, aka ", rd_index);
            set_reg_by_index(cpu, rd_index, result);
            log_finln!(", and has been set to:");
            log!(t, 6, "     {:#06X} | {:#018b} | unsigned {}", result, result, result);
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
fn set_reg_by_index(cpu: &mut CPUState, rd: u8, value: u16) {
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
fn alu_operation(cpu: &mut CPUState, upper_nibble: u8, operand1: u16, operand2: u16) -> u16 {//Needs mutable reference to CPUState to sets flags properly
    //TODO set flags correctly
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
            return operand1;//No need for any flags to be set with store
        },
        _ => {//TODO should we do some sort of error handling for this, or do we need to jump somewhere if this occurs?
            log_finln!("(invalid)");
            return 0;
        },
    }
}
