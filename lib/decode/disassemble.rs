/* NAME//TODO
 * By: John Jekel
 *
 * TODO description
 *
*/

/* Imports */

use crate::decode::DecodedInstruction;
use crate::decode::DecodedInstruction::*;

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

pub fn disassemble_jekel_style(decoded_inst: &DecodedInstruction) -> String {
    match decoded_inst {
        DSI6{imm6} => { return format!("dsi6 {:#04X}", *imm6); },
        CALL{..} => { return "todo".to_string(); },
        JMPF{..} => { return "todo".to_string(); },
        JMPR{..} => { return "jmpr".to_string(); },
        FIR_MOV{..}=> { return "todo".to_string(); },
        Fraction{..} => { return "todo".to_string(); },
        INT_SET{..} => { return "todo".to_string(); },
        IRQ{..} => { return "todo".to_string(); },
        SECBANK{..} => { return "todo".to_string(); },
        FIQ{..} => { return "todo".to_string(); },
        IRQ_Nest_Mode{..} => { return "todo".to_string(); },
        BREAK{..} => { return "break".to_string(); },
        CALLR{..} => { return "callr".to_string(); },
        DIVS{..} => { return "divs".to_string(); },
        DIVQ{..} => { return "divq".to_string(); },
        EXP{..} => { return "exp".to_string(); },
        NOP{..} => { return "nop".to_string(); },
        DS_Access{..} => { return "todo".to_string(); },
        FR_Access{..} => { return "todo".to_string(); },
        MUL{..} => { return "todo".to_string(); },
        MULS{..} => { return "todo".to_string(); },
        Register_BITOP_Rs{..} => { return "todo".to_string(); },
        Register_BITOP_offset{..} => { return "todo".to_string(); },
        Memory_BITOP_offset{..} => { return "todo".to_string(); },
        Memory_BITOP_Rs{..} => { return "todo".to_string(); },
        sixteen_bits_Shift{..} => { return "todo".to_string(); },
        RETI{..} => { return "reti".to_string(); },
        RETF{..} => { return "retf".to_string(); },
        Base_plus_Disp6{..} => { return "todo".to_string(); },
        IMM6{..} => { return "todo".to_string(); },
        Branch{..} => { return "todo".to_string(); },
        Stack_Operation{..} => { return "todo".to_string(); },
        DS_Indirect{..} => { return "todo".to_string(); },
        IMM16{..} => { return "todo".to_string(); },
        Direct16{..} => { return "todo".to_string(); },
        Direct6{..} => { return "todo".to_string(); },
        Register{..} => { return "todo".to_string(); },

        Invalid => { return "(invalid)".to_string(); },
    }
}

pub fn disassemble_generalplus_style(decoded_inst: &DecodedInstruction) -> String {
    match decoded_inst {
        DSI6{imm6} => { return format!("DS = {:#04X}", *imm6); },
        CALL{..} => { return "todo".to_string(); },
        JMPF{..} => { return "todo".to_string(); },
        JMPR{..} => { return "GOTO MR".to_string(); },
        FIR_MOV{..}=> { return "todo".to_string(); },
        Fraction{..} => { return "todo".to_string(); },
        INT_SET{..} => { return "todo".to_string(); },
        IRQ{..} => { return "todo".to_string(); },
        SECBANK{..} => { return "todo".to_string(); },
        FIQ{..} => { return "todo".to_string(); },
        IRQ_Nest_Mode{..} => { return "todo".to_string(); },
        BREAK{..} => { return "BREAK".to_string(); },
        CALLR{..} => { return "CALL MR".to_string(); },
        DIVS{..} => { return "DIVS MR, R2".to_string(); },
        DIVQ{..} => { return "DIVQ MR, R2".to_string(); },
        EXP{..} => { return "R2 = EXP R4".to_string(); },
        NOP{..} => { return "NOP".to_string(); },
        DS_Access{..} => { return "todo".to_string(); },
        FR_Access{..} => { return "todo".to_string(); },
        MUL{..} => { return "todo".to_string(); },
        MULS{..} => { return "todo".to_string(); },
        Register_BITOP_Rs{..} => { return "todo".to_string(); },
        Register_BITOP_offset{..} => { return "todo".to_string(); },
        Memory_BITOP_offset{..} => { return "todo".to_string(); },
        Memory_BITOP_Rs{..} => { return "todo".to_string(); },
        sixteen_bits_Shift{..} => { return "todo".to_string(); },
        RETI{..} => { return "RETI".to_string(); },
        RETF{..} => { return "RETF".to_string(); },
        Base_plus_Disp6{..} => { return "todo".to_string(); },
        IMM6{..} => { return "todo".to_string(); },
        Branch{..} => { return "todo".to_string(); },
        Stack_Operation{..} => { return "todo".to_string(); },
        DS_Indirect{..} => { return "todo".to_string(); },
        IMM16{..} => { return "todo".to_string(); },
        Direct16{..} => { return "todo".to_string(); },
        Direct6{..} => { return "todo".to_string(); },
        Register{..} => { return "todo".to_string(); },

        Invalid => { return "(invalid)".to_string(); },
    }
}

pub fn disassemble_mame_style(decoded_inst: &DecodedInstruction) -> String {
    use super::common::reg_string_lower;
    use super::common::bit_op_string_lower;
    match decoded_inst {
        DSI6{imm6} => { return format!("ds = {:04x}", *imm6); },
        CALL{a22} => { return format!("call {:06x}", *a22); },
        JMPF{a22} => { return format!("goto {:06x}", *a22); },
        JMPR{..} => { return "goto mr".to_string(); },
        FIR_MOV{fir}=> { return format!("fir_mov {}", if *fir { "on" } else { "off" }); },
        Fraction{fra} => { return format!("fraction {}", if *fra { "on" } else { "off" }); },
        INT_SET{f, i} => {
            let operand: &str;
            match (f, i) {
                (false, false) => { operand = "off"; },
                (false, true) => { operand = "irq"; },
                (true, false) => { operand = "fiq"; },
                (true, true) => { operand = "fiq,irq"; },
            }
            return format!("int {}", operand);
        },
        IRQ{i} => { return format!("irq {}", if *i { "on" } else { "off" }); },
        SECBANK{s} => { return format!("secbank {}", if *s { "on" } else { "off" }); },
        FIQ{f} => { return format!("fiq {}", if *f { "on" } else { "off" }); },
        IRQ_Nest_Mode{n} => { return format!("irqnest {}", if *n { "on" } else { "off" }); },
        BREAK{..} => { return "break".to_string(); },
        CALLR{..} => { return "call mr".to_string(); },
        DIVS{..} => { return "divs mr, r2".to_string(); },
        DIVQ{..} => { return "divq mr, r2".to_string(); },
        EXP{..} => { return "r2 = exp r4".to_string(); },
        NOP{..} => { return "nop".to_string(); },
        DS_Access{w, rs} => {
            let reg = reg_string_lower!(*rs);
            if *w {
                return format!("ds = {}", reg);
            } else {
                return format!("{} = ds", reg);
            }
        },
        FR_Access{w, rs} => {
            let reg = reg_string_lower!(*rs);
            if *w {
                return format!("fr = {}", reg);
            } else {
                return format!("{} = fr", reg);
            }
        },
        MUL{s_rs, rd, s_rd, rs} => {
            return format!("MR = {}*{}, {}{}",
                reg_string_lower!(*rd),
                reg_string_lower!(*rs),
                if *s_rd { "s" } else { "u" },
                if *s_rs { "s" } else { "u" },
            );
        },
        MULS{s_rs, rd, s_rd, size, rs} => {
            return format!("MR = [{}]*[{}], {}{}, {}",
                reg_string_lower!(*rd),
                reg_string_lower!(*rs),
                if *s_rd { "s" } else { "u" },
                if *s_rs { "s" } else { "u" },
                size,
            );
        },
        Register_BITOP_Rs{rd, op, rs} => {
            return format!("{} {},{}",
                bit_op_string_lower!(*op),
                reg_string_lower!(*rd),
                reg_string_lower!(*rs),
            );
        },
        Register_BITOP_offset{rd, op, offset} => {
            return format!("{} {},{}",
                bit_op_string_lower!(*op),
                reg_string_lower!(*rd),
                offset,
            );
        },
        Memory_BITOP_offset{..} => { return "TODO".to_string(); },
        Memory_BITOP_Rs{..} => { return "TODO".to_string(); },
        sixteen_bits_Shift{..} => { return "TODO".to_string(); },
        RETI{..} => { return "reti".to_string(); },
        RETF{..} => { return "retf".to_string(); },
        Base_plus_Disp6{..} => { return "TODO".to_string(); },
        IMM6{..} => { return "TODO".to_string(); },
        Branch{..} => { return "TODO".to_string(); },
        Stack_Operation{..} => { return "TODO".to_string(); },
        DS_Indirect{..} => { return "TODO".to_string(); },
        IMM16{..} => { return "TODO".to_string(); },
        Direct16{..} => { return "TODO".to_string(); },
        Direct6{..} => { return "TODO".to_string(); },
        Register{..} => { return "TODO".to_string(); },

        Invalid => { return "--".to_string(); },
    }
}
