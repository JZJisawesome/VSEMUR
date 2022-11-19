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

macro_rules! branch_op_string_mame {
    ($op:expr) => {{
        let string: &str;
        {
            use crate::decode::DecodedBranchOp::*;
            match $op {
                JCC_JB_JNAE => { string = "jb"; },
                JCS_JNB_JAE => { string = "jae"; },
                JSC_JGE_JNL => { string = "jge"; },
                JSS_JNGE_JL => { string = "jl"; },
                JNE_JNZ => { string = "jne"; },
                JZ_JE => { string = "je"; },
                JPL => { string = "jpl"; },
                JMI => { string = "jmi"; },
                JBE_JNA => { string = "jbe"; },
                JNBE_JA => { string = "ja"; },
                JLE_JNG => { string = "jle"; },
                JNLE_JG => { string = "jg"; },
                JVC => { string = "jvc"; },
                JVS => { string = "jvs"; },
                JMP => { string = "jmp"; },

                Invalid => { string = "(invalid)"; }
            }
        }
        string
    }};
}

macro_rules! auto_alu_op_string_mame {
    ($op:expr) => {{
        let string: &str;
        {
            use crate::decode::DecodedALUOp::*;
            match $op {
                ADD | ADC => { string = "+="; },
                SUB | SBC => { string = "-="; },
                NEG => { string = "=-"; }
                XOR => { string = "^="; },
                LOAD => { string = "="; },
                OR => { string = "|="; },
                AND => { string = "&="; },
                CMP | TEST | STORE => { string = debug_panic!(""); },

                Invalid => { string = "(invalid)"; },
            }
        }
        string
    }};
}

macro_rules! carry_string_if_carry_mame {
    ($op:expr) => {{
        let string: &str;
        {
            use crate::decode::DecodedALUOp::*;
            match $op {
                ADC | SBC => { string = ", carry"; }
                _ => { string = ""; }
            }
        }
        string
    }};
}

macro_rules! sft_op_amount_string_if_not_nop_mame {
    ($sft:expr, $shift_amount:expr) => {{//shift_amount is sfc + 1
        let string: String;
        {
            use crate::decode::DecodedSFTOp::*;
            if matches!($sft, NOP) {
                string = "".to_string();
            } else {
                string = format!(" {} {}", sft_op_string_lower!($sft), $shift_amount);
            }
        }
        string
    }};
}

/* Static Variables */

//TODO

/* Types */

//TODO

/* Associated Functions and Methods */

//TODO

/* Functions */

//TODO move these disassembly functions to their own modules

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
pub fn disassemble_mame_style(addr: u32, decoded_inst: &DecodedInstruction) -> String {
    use super::common::*;
    match decoded_inst {
        DSI6{imm6} => { return format!("ds = {:02x}", *imm6); },
        CALL{a22} => { return format!("call {:06x}", *a22); },
        JMPF{a22} => { return format!("goto {:06x}", *a22); },
        JMPR{..} => { return "goto mr".to_string(); },
        FIR_MOV{fir}=> { return format!("fir_mov {}", if *fir { "off" } else { "on" }); },//That's confusing
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
            //NOTE: When comparing to MAME we seem to miss some of these
            //However we're actually correct. Consider 0xFC13 for example.
            //Since bit 3 is 0 and not 1, this means the instruction is not MUL (there are other ways to tell too)
            //Thus this instruction is invalid in this case, but MAME thinks it's MR = sr*r3, us
            //To be fair, we also think that some invalid instructions are valid MULs while MAME correctly catches them as invalid (ex. 0xFF37)
            //Since those instructions should never be executed, it really don't matter though
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
                if *size == 0 { 16 } else { *size },
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
            let operator: &str = auto_alu_op_string_mame!(*op);

            //Assemble everything together
            return format!("{} {} [bp+{:02x}]{}",
                reg_string_lower!(*rd),
                operator,
                *imm6,
                carry_string_if_carry_mame!(*op),
            );
        },
        IMM6{op, rd, imm6} => {
            use super::DecodedALUOp::*;

            //Handle special cases
            match *op {
                STORE => { return "--".to_string(); },
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
            let operator: &str = auto_alu_op_string_mame!(*op);

            //Assemble everything together
            return format!("{} {} {:02x}{}",
                reg_string_lower!(*rd),
                operator,
                *imm6,
                carry_string_if_carry_mame!(*op),
            );
        },
        Branch{op, d, imm6} => {
            return format!("{} {:04x}",
                branch_op_string_mame!(*op),
                if *d { addr - (*imm6 as u32) + 1 } else { addr + (*imm6 as u32) + 1 },
            );
        },//FIXME what about wrapping (underflow)?
        Stack_Operation{op, rd_index, size, rs} => {
            use super::DecodedStackOp::*;

            let first_index;
            let second_index;
            match *op {
                PUSH => {
                    if *size <= (*rd_index + 1) {
                        first_index = *rd_index + 1 - *size;
                        second_index = *rd_index;
                    } else {
                        return "--".to_string();
                    }
                },
                POP => {
                    first_index = *rd_index + 1;
                    second_index = *rd_index + *size;
                },
                _ => { return "--".to_string(); }
            }

            if (first_index > 0b111) || (second_index > 0b111) {
                return "--".to_string();
            }

            return format!("{} {}, {} {} [{}]",
                stack_op_string_lower!(*op),
                reg_string_by_index_lower!(first_index),
                reg_string_by_index_lower!(second_index),
                if matches!(op, PUSH) { "to" } else { "from" },
                reg_string_lower!(*rs),
            );
        },
        DS_Indirect{op, rd, d, at, rs} => {
            use super::DecodedALUOp::*;

            let ds_string_if_d: &str = if *d { "ds:" } else { "" };
            let mut rs_string: String = reg_string_lower!(*rs).to_string();
            match *at {
                super::DecodedAtOp::NOP => {},//Just leave it as-is
                super::DecodedAtOp::PostDecrement => { rs_string.push_str("--"); },
                super::DecodedAtOp::PostIncrement => { rs_string.push_str("++"); },
                super::DecodedAtOp::PreIncrement => { rs_string = "++".to_string() + &rs_string; },
                super::DecodedAtOp::Invalid => { debug_panic!(); }
            }

            //Handle special cases
            match *op {
                STORE => {
                    return format!("{}[{}] = {}",
                        ds_string_if_d,
                        rs_string,
                        reg_string_lower!(*rd),
                    );
                },
                CMP => {
                    return format!("cmp {}, {}[{}]",
                        reg_string_lower!(*rd),
                        ds_string_if_d,
                        rs_string,
                    );
                },
                TEST => {
                    return format!("test {}, {}[{}]",
                        reg_string_lower!(*rd),
                        ds_string_if_d,
                        rs_string,
                    );
                },
                _ => {},//Continue on
            }

            //Normal ones: get the operator
            let operator: &str = auto_alu_op_string_mame!(*op);

            //Assemble everything together
            return format!("{} {} {}[{}]{}",
                reg_string_lower!(*rd),
                operator,
                ds_string_if_d,
                rs_string,
                carry_string_if_carry_mame!(*op),
            );
        },
        IMM16{op, rd, rs, imm16} => {
            use super::DecodedALUOp::*;

            //Handle special cases
            match *op {
                LOAD => {
                    return format!("{} = {:04x}",
                        reg_string_lower!(*rd),
                        *imm16,
                    );
                },
                STORE => { return "--".to_string(); },
                NEG => {
                    return format!("{} = -{:04x}",
                        reg_string_lower!(*rd),
                        *imm16,
                    );
                },
                CMP => {
                    return format!("cmp {}, {:04x}",
                        reg_string_lower!(*rs),//TODO is this rs or rd?
                        *imm16,
                    );
                },
                TEST => {
                    return format!("test {}, {:04x}",
                        reg_string_lower!(*rs),//TODO is this rs or rd?
                        *imm16,
                    );
                },
                _ => {},//Continue on
            }

            //Normal ones: get the operator
            let operator: &str;
            match *op {
                ADD | ADC => { operator = "+"; },
                SUB | SBC => { operator = "-"; },
                XOR => { operator = "^"; },
                OR => { operator = "|"; },
                AND => { operator = "&"; },
                NEG | LOAD | CMP | TEST | STORE => { operator = debug_panic!(""); },

                Invalid => { operator = "(invalid)"; },
            }

            //Assemble everything together
            return format!("{} = {} {} {:04x}{}",
                reg_string_lower!(*rd),
                reg_string_lower!(*rs),
                operator,
                *imm16,
                carry_string_if_carry_mame!(*op),
            );
        },
        Direct16{op, rd, w, rs, a16} => {
            use super::DecodedALUOp::*;

            //Handle special cases
            match *op {
                LOAD => {
                    if *w {
                        return "--".to_string();
                    } else {
                        return format!("{} = [{:04x}]",
                            reg_string_lower!(*rd),
                            *a16,
                        );
                    }
                },
                STORE => {
                    if *w {
                        return format!("[{:04x}] = {}",
                            *a16,
                            reg_string_lower!(*rs),
                        );
                    } else {
                        return "--".to_string();
                    }
                },
                NEG => {
                    if *w {
                        return format!("[{:04x}] = -{}",
                            *a16,
                            reg_string_lower!(*rs),
                        );
                    } else {
                        return format!("{} = -[{:04x}]",
                            reg_string_lower!(*rs),
                            *a16,
                        );
                    }
                },
                CMP => {
                    if *w {
                        return "TODO".to_string();//In this case, should it be cmp Rd, Rs?
                    } else {
                        return format!("cmp {}, [{:04x}]",
                            reg_string_lower!(*rs),
                            *a16,
                        );
                    }
                },
                TEST => {
                    if *w {
                        return "TODO".to_string();//In this case, should it be test Rd, Rs?
                    } else {
                        return format!("test {}, [{:04x}]",
                            reg_string_lower!(*rs),
                            *a16,
                        );
                    }
                },
                _ => {},//Continue on
            }

            //Normal ones: get the operator
            let operator: &str;
            match *op {
                ADD | ADC => { operator = "+"; },
                SUB | SBC => { operator = "-"; },
                XOR => { operator = "^"; },
                OR => { operator = "|"; },
                AND => { operator = "&"; },
                NEG | LOAD | CMP | TEST | STORE => { operator = debug_panic!(""); },

                Invalid => { operator = "(invalid)"; },
            }

            //Assemble everything together
            if *w {
                //NOTE: MAME says that rs should come before rd, but I believe this is a bug in MAME
                //Ex, the instruction 0xb91b 0x2075 is AND, has the W bit set, rd is 0b100, rs is 0b011, and A16 is 0x2075
                //We print [2075] = r4 & r3, but MAME prints [2075] = r3 & r4
                //This dosn't matter for and, but for things like CMP, subtraction, etc. it is a problem
                return format!("[{:04x}] = {} {} {}{}",
                    *a16,
                    reg_string_lower!(*rd),
                    operator,
                    reg_string_lower!(*rs),
                    carry_string_if_carry_mame!(*op),
                );
            } else {
                return format!("{} = {} {} [{:04x}]{}",
                    reg_string_lower!(*rd),
                    reg_string_lower!(*rs),
                    operator,
                    *a16,
                    carry_string_if_carry_mame!(*op),
                );
            }
        },
        Direct6{op, rd, a6} => {
            use super::DecodedALUOp::*;

            //Handle special cases
            match *op {
                STORE => {
                    return format!("[{:02x}] = {}",
                        *a6,
                        reg_string_lower!(*rd),
                    );
                },
                CMP => {
                    return format!("cmp {}, [{:02x}]",
                        reg_string_lower!(*rd),
                        *a6,
                    );
                },
                TEST => {
                    return format!("test {}, [{:02x}]",
                        reg_string_lower!(*rd),
                        *a6,
                    );
                },
                _ => {},//Continue on
            }

            //Normal ones: get the operator
            let operator: &str = auto_alu_op_string_mame!(*op);

            //Assemble everything together
            return format!("{} {} [{:02x}]{}",
                reg_string_lower!(*rd),
                operator,
                *a6,
                carry_string_if_carry_mame!(*op),
            );
        },
        Register{op, rd, sft, sfc, rs} => {
            use super::DecodedALUOp::*;

            let shift_amount = sfc + 1;

            //Handle special cases
            match *op {
                STORE => { return "--".to_string(); },
                CMP => {
                    return format!("cmp {}, {}{}",
                        reg_string_lower!(*rd),
                        reg_string_lower!(*rs),
                        sft_op_amount_string_if_not_nop_mame!(*sft, shift_amount)
                    );
                },
                TEST => {
                    return format!("test {}, {}{}",
                        reg_string_lower!(*rd),
                        reg_string_lower!(*rs),
                        sft_op_amount_string_if_not_nop_mame!(*sft, shift_amount)
                    );
                },
                _ => {},//Continue on
            }

            //Normal ones: get the operator
            let operator: &str = auto_alu_op_string_mame!(*op);

            //Assemble everything together
            return format!("{} {} {}{}{}",
                reg_string_lower!(*rd),
                operator,
                reg_string_lower!(*rs),
                sft_op_amount_string_if_not_nop_mame!(*sft, shift_amount),
                carry_string_if_carry_mame!(*op),
            );
        },

        Invalid => { return "--".to_string(); },
    }
}
