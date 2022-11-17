/* NAME//TODO
 * By: John Jekel
 *
 * TODO description
 *
*/

/* Imports */

use crate::debug_panic;
use crate::decode::DecodedInstruction;
use crate::decode::DecodedInstruction::*;

/* Constants */

//TODO

/* Macros */

/* Static Variables */

//TODO

/* Types */

//TODO

/* Associated Functions and Methods */

//TODO

/* Functions */

pub fn disassemble_jekel_style(decoded_inst: &DecodedInstruction) -> String {//WIP
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

pub fn disassemble_generalplus_style(decoded_inst: &DecodedInstruction) -> String {//WIP
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

//Not perfect (ex. because unlike MAME we only really have one kind of bad instruction), but aims to be reasonably close
pub fn disassemble_mame_style(decoded_inst: &DecodedInstruction) -> String {
    use super::common::*;
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
        Memory_BITOP_offset{rd, d, op, offset} => {
            return format!("{} {}[{}],{}",
                bit_op_string_lower!(*op),
                if *d { "ds:" } else { "" },
                reg_string_lower!(*rd),
                offset,
            );
        },
        Memory_BITOP_Rs{rd, d, op, rs} => {
            return format!("{} {}[{}],{}",
                bit_op_string_lower!(*op),
                if *d { "ds:" } else { "" },
                reg_string_lower!(*rd),
                reg_string_lower!(*rs),
            );
        },
        sixteen_bits_Shift{rd, op, rs} => {
            return format!("{0} = {0} {1} {2}",
                reg_string_lower!(*rd),
                lsft_op_string_lower!(*op),
                reg_string_lower!(*rs),
            );
        },
        RETI{..} => { return "reti".to_string(); },
        RETF{..} => { return "retf".to_string(); },
        Base_plus_Disp6{op, rd, imm6} => {
            use super::DecodedALUOp::*;

            //Handle special cases
            match *op {
                STORE => {
                    return format!("[bp+{:02x}] = {}",
                        *imm6,
                        reg_string_lower!(*rd),
                    );
                },
                CMP => {
                    return format!("cmp {}, [bp+{:02x}]",
                        reg_string_lower!(*rd),
                        *imm6,
                    );
                },
                TEST => {
                    return format!("test {}, [bp+{:02x}]",
                        reg_string_lower!(*rd),
                        *imm6,
                    );
                },
                _ => {},//Continue on
            }

            //Normal ones: get the operator
            let operator: &str;
            match *op {
                ADD | ADC => { operator = "+="; },
                SUB | SBC => { operator = "-="; },
                NEG => { operator = "=-"; }
                XOR => { operator = "^="; },
                LOAD => { operator = "="; },
                OR => { operator = "|="; },
                AND => { operator = "&="; },
                CMP | TEST | STORE => { operator = debug_panic!(""); },

                Invalid => { operator = "(invalid)"; },
            }

            //Determine if we need to append , carry to the end
            let carry: bool;
            match *op {
                ADC | SBC => { carry = true; }
                _ => { carry = false; }
            }

            //Assemble everything together
            return format!("{} {} [bp+{:02x}]{}",
                reg_string_lower!(*rd),
                operator,
                *imm6,
                if carry { ", carry" } else { "" },
            );
        },
        IMM6{op, rd, imm6} => {
            use super::DecodedALUOp::*;

            //Handle special cases
            match *op {
                STORE => {
                    return "<BAD>".to_string();
                },
                CMP => {
                    return format!("cmp {}, {:02x}",
                        reg_string_lower!(*rd),
                        *imm6,
                    );
                },
                TEST => {
                    return format!("test {}, {:02x}",
                        reg_string_lower!(*rd),
                        *imm6,
                    );
                },
                _ => {},//Continue on
            }

            //Normal ones: get the operator
            let operator: &str;
            match *op {
                ADD | ADC => { operator = "+="; },
                SUB | SBC => { operator = "-="; },
                NEG => { operator = "=-"; }
                XOR => { operator = "^="; },
                LOAD => { operator = "="; },
                OR => { operator = "|="; },
                AND => { operator = "&="; },
                CMP | TEST | STORE => { operator = debug_panic!(""); },

                Invalid => { operator = "(invalid)"; },
            }

            //Determine if we need to append , carry to the end
            let carry: bool;
            match *op {
                ADC | SBC => { carry = true; }
                _ => { carry = false; }
            }

            //Assemble everything together
            return format!("{} {} {:02x}{}",
                reg_string_lower!(*rd),
                operator,
                *imm6,
                if carry { ", carry" } else { "" },
            );
        },
        Branch{..} => { return "Branch TODO".to_string(); },
        Stack_Operation{..} => { return "Stack_Operation TODO".to_string(); },
        DS_Indirect{..} => { return "DS_Indirect TODO".to_string(); },
        IMM16{..} => { return "IMM16 TODO".to_string(); },
        Direct16{..} => { return "Direct16 TODO".to_string(); },
        Direct6{..} => { return "Direct6 TODO".to_string(); },
        Register{..} => { return "Register TODO".to_string(); },

        Invalid => { return "--".to_string(); },
    }
}
