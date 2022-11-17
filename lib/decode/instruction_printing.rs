/* NAME//TODO
 * By: John Jekel
 *
 * TODO description
 *
*/

/* Imports */

//TODO

/* Constants */

//TODO

/* Macros */

macro_rules! log_data {
    ($indent:expr, $pretext:expr, $data:expr) => {
        log!($indent, "{}: {1:#04X} | {1:#08b} | unsigned {1}", $pretext, $data);
    };
}
pub(super) use log_data;

macro_rules! log_addr {
    ($indent:expr, $pretext:expr, $addr:expr) => {
        log!($indent, "{}: {:#04X}_{:04X}", $pretext, $addr >> 16, $addr & 0xFFFF);
    };
}
pub(super) use log_addr;

macro_rules! log_inst {
    ($indent:expr, $decoded_instruction: expr) => {
        //Compile times were getting a bit too long due to the large macro inlining in decode
        if cfg!(debug_assertions) {
            crate::decode::instruction_printing::log_inst_func($indent, $decoded_instruction);
        }
    };
}
pub(super) use log_inst;

macro_rules! alu_op_string {
    ($op:expr) => {{
        let string: &str;
        {
            use crate::decode::DecodedALUOp::*;
            match $op {
                ADD => { string = "ADD"; },
                ADC => { string = "ADC"; },
                SUB => { string = "SUB"; },
                SBC => { string = "SBC"; },
                CMP => { string = "CMP"; },
                NEG => { string = "NEG"; },
                XOR => { string = "XOR"; },
                LOAD => { string = "LOAD"; },
                OR => { string = "OR"; },
                AND => { string = "AND"; },
                TEST => { string = "TEST"; },
                STORE => { string = "STORE"; },

                Invalid => { string = "(invalid)"; }
            }
        }
        string
    }};
}

macro_rules! branch_op_string {
    ($op:expr) => {{
        let string: &str;
        {
            use crate::decode::DecodedBranchOp::*;
            match $op {
                JCC_JB_JNAE => { string = "JCC/JB/JNAE"; },
                JCS_JNB_JAE => { string = "JCS/JNB/JAE"; },
                JSC_JGE_JNL => { string = "JSC/JGE/JNL"; },
                JSS_JNGE_JL => { string = "JSS/JNGE/JL"; },
                JNE_JNZ => { string = "JNE/JNZ"; },
                JZ_JE => { string = "JZ/JE"; },
                JPL => { string = "JPL"; },
                JMI => { string = "JMI"; },
                JBE_JNA => { string = "JBE/JNA"; },
                JNBE_JA => { string = "JNBE/JA"; },
                JLE_JNG => { string = "JLE/JNG"; },
                JNLE_JG => { string = "JNLE/JG"; },
                JVC => { string = "JVC"; },
                JVS => { string = "JVS"; },
                JMP => { string = "JMP"; },

                Invalid => { string = "(invalid)"; }
            }
        }
        string
    }};
}

macro_rules! stack_op_string {
    ($op:expr) => {{
        let string: &str;
        {
            use crate::decode::DecodedStackOp::*;
            match $op {
                PUSH => { string = "PUSH"; },
                POP => { string = "POP"; },

                Invalid => { string = "(invalid)"; }
            }
        }
        string
    }};
}

macro_rules! at_op_string {
    ($op:expr) => {{
        let string: &str;
        {
            use crate::decode::DecodedAtOp::*;
            match $op {
                NOP => { string = "Rs"; },
                PostDecrement => { string = "Rs--"; },
                PostIncrement => { string = "Rs++"; },
                PreIncrement => { string = "++Rs"; },

                Invalid => { string = "(invalid)"; }
            }
        }
        string
    }};
}

macro_rules! sft_op_string {
    ($op:expr) => {{
        let string: &str;
        {
            use crate::decode::DecodedSFTOp::*;
            match $op {
                NOP => { string = "NOP"; },
                ASR => { string = "ASR"; },
                LSL => { string = "LSL"; },
                LSR => { string = "LSR"; },
                ROL => { string = "ROL"; },
                ROR => { string = "ROR"; },

                Invalid => { string = "(invalid)"; }
            }
        }
        string
    }};
}

//TODO (also pub(crate) use the_macro statements here too)

/* Static Variables */

//TODO

/* Types */

//TODO

/* Associated Functions and Methods */

//TODO

/* Functions */

pub(super) fn log_inst_func(indent: u8, decoded_inst: &crate::decode::DecodedInstruction) {
    use crate::logging::log;
    use crate::logging::log_noln;
    use crate::logging::log_finln;

    use super::DecodedInstruction::*;
    use super::common::*;

    log_noln!(indent, "Instruction Type: ");
    if cfg!(debug_assertions) {//TODO print sub fields of each type too (on new lines indented under it)
        match decoded_inst {
            DSI6{imm6} => {
                log_finln!("DSI6");
                log_data!(indent + 1, "imm6", *imm6);
            },
            CALL{a22} => {
                log_finln!("CALL");
                log_addr!(indent + 1, "a22", *a22);
            },
            JMPF{a22} => {
                log_finln!("JMPF");
                log_addr!(indent + 1, "a22", *a22);
            },
            JMPR => { log_finln!("JMPR"); },
            FIR_MOV{fir}=> {
                log_finln!("FIR_MOV");
                log!(indent + 1, "FIR: {}", *fir);
            },
            Fraction{fra} => {
                log_finln!("Fraction");
                log!(indent + 1, "FRA: {}", *fra);
            },
            INT_SET{f, i} => {
                log_finln!("INT SET");
                log!(indent + 1, "F: {}", *f);
                log!(indent + 1, "I: {}", *i);
            },
            IRQ{i} => {
                log_finln!("IRQ");
                log!(indent + 1, "I: {}", *i);
            },
            SECBANK{s} => {
                log_finln!("SECBANK");
                log!(indent + 1, "S: {}", *s);
            },
            FIQ{f} => {
                log_finln!("FIQ");
                log!(indent + 1, "F: {}", *f);
            },
            IRQ_Nest_Mode{n} => {
                log_finln!("IRQ Nest Mode");
                log!(indent + 1, "N: {}", *n);
            },
            BREAK => { log_finln!("BREAK"); },
            CALLR => { log_finln!("CALLR"); },
            DIVS => { log_finln!("DIVS"); },
            DIVQ => { log_finln!("DIVQ"); },
            EXP => { log_finln!("EXP"); },
            NOP => { log_finln!("NOP"); },
            DS_Access{w, rs} => {
                log_finln!("DS Access");
                log!(indent + 1, "W: {}", *w);
                log!(indent + 1, "Rs: {}", reg_string!(*rs));
            },
            FR_Access{w, rs} => {
                log_finln!("FR Access");
                log!(indent + 1, "W: {}", *w);
                log!(indent + 1, "Rs: {}", reg_string!(*rs));
            },
            MUL{s_rs, rd, s_rd, rs} => {
                log_finln!("MUL");
                log!(indent + 1, "S_Rs: {}", *s_rs);
                log!(indent + 1, "Rd: {}", reg_string!(*rd));
                log!(indent + 1, "S_Rd: {}", *s_rd);
                log!(indent + 1, "Rs: {}", reg_string!(*rs));
            },
            MULS{s_rs, rd, s_rd, size, rs} => {
                log_finln!("MULS");
                log!(indent + 1, "S_Rs: {}", *s_rs);
                log!(indent + 1, "Rd: {}", reg_string!(*rd));
                log!(indent + 1, "S_Rd: {}", *s_rd);
                log_data!(indent + 1, "Size", *size);
                log!(indent + 1, "Rs: {}", reg_string!(*rs));
            },
            Register_BITOP_Rs{rd, op, rs} => {
                log_finln!("Register BITOP (Rs)");
                log!(indent + 1, "Rd: {}", reg_string!(*rd));
                log!(indent + 1, "Bitop: {}", bit_op_string!(*op));
                log!(indent + 1, "Rs: {}", reg_string!(*rs));
            },
            Register_BITOP_offset{rd, op, offset} => {
                log_finln!("Register BITOP (offset)");
                log!(indent + 1, "Rd: {}", reg_string!(*rd));
                log!(indent + 1, "Bitop: {}", bit_op_string!(*op));
                log_data!(indent + 1, "Offset", *offset);
            },
            Memory_BITOP_offset{rd, op, d, offset} => {
                log_finln!("Memory BITOP (offset)");
                log!(indent + 1, "Rd: {}", reg_string!(*rd));
                log!(indent + 1, "Bitop: {}", bit_op_string!(*op));
                log!(indent + 1, "D: {}", *d);
                log_data!(indent + 1, "Offset", *offset);
            },
            Memory_BITOP_Rs{rd, op, d, rs} => {
                log_finln!("Memory BITOP (Rs)");
                log!(indent + 1, "Rd: {}", reg_string!(*rd));
                log!(indent + 1, "Bitop: {}", bit_op_string!(*op));
                log!(indent + 1, "D: {}", *d);
                log!(indent + 1, "Rs: {}", reg_string!(*rs));
            },
            sixteen_bits_Shift{rd, op, rs} => {
                log_finln!("16 bits Shift");
                log!(indent + 1, "Rd: {}", reg_string!(*rd));
                log!(indent + 1, "OP: {}", lsft_op_string!(*op));
                log!(indent + 1, "Rs: {}", reg_string!(*rs));
            },
            RETI => { log_finln!("RETI"); },
            RETF => { log_finln!("RETF"); },
            Base_plus_Disp6{op, rd, imm6} => {
                log_finln!("Base+Disp6");
                log!(indent + 1, "OP: {}", alu_op_string!(*op));
                log!(indent + 1, "Rd: {}", reg_string!(*rd));
                log_data!(indent + 1, "IMM6", *imm6);
            },
            IMM6{op, rd, imm6} => {
                log_finln!("IMM6");
                log!(indent + 1, "OP: {}", alu_op_string!(*op));
                log!(indent + 1, "Rd: {}", reg_string!(*rd));
                log_data!(indent + 1, "IMM6", *imm6);
            },
            Branch{op, d, imm6} => {
                log_finln!("Branch");
                log!(indent + 1, "OP: {}", branch_op_string!(*op));
                log!(indent + 1, "D: {}", *d);
                log_data!(indent + 1, "IMM6", *imm6);
            },
            Stack_Operation{op, rd_index, size, rs} => {
                log_finln!("Stack Operation");
                log!(indent + 1, "OP: {}", stack_op_string!(*op));
                log!(indent + 1, "Rh/Rd index: {}", *rd_index);
                log_data!(indent + 1, "Size", *size);
                log!(indent + 1, "Rs: {}", reg_string!(*rs));
            },
            DS_Indirect{op, rd, d, at, rs} => {
                log_finln!("DS_Indirect");
                log!(indent + 1, "OP: {}", alu_op_string!(*op));
                log!(indent + 1, "Rd: {}", reg_string!(*rd));
                log!(indent + 1, "D: {}", *d);
                log!(indent + 1, "@: {}", at_op_string!(*at));
                log!(indent + 1, "Rs: {}", reg_string!(*rs));
            },
            IMM16{op, rd, rs, imm16} => {
                log_finln!("IMM16");
                log!(indent + 1, "OP: {}", alu_op_string!(*op));
                log!(indent + 1, "Rd: {}", reg_string!(*rd));
                log!(indent + 1, "Rs: {}", reg_string!(*rs));
                log_data!(indent + 1, "IMM16", *imm16);
            },
            Direct16{op, rd, rs, w, a16} => {
                log_finln!("Direct16");
                log!(indent + 1, "OP: {}", alu_op_string!(*op));
                log!(indent + 1, "Rd: {}", reg_string!(*rd));
                log!(indent + 1, "Rs: {}", reg_string!(*rs));
                log!(indent + 1, "W: {}", *w);
                log!(indent + 1, "A16: {:#06X}", *a16);
            },
            Direct6{op, rd, a6} => {
                log_finln!("Direct6");
                log!(indent + 1, "OP: {}", alu_op_string!(*op));
                log!(indent + 1, "Rd: {}", reg_string!(*rd));
                log!(indent + 1, "A6: {:#04X}", *a6);
            },
            Register{op, rd, sft, sfc, rs} => {
                log_finln!("Register");
                log!(indent + 1, "OP: {}", alu_op_string!(*op));
                log!(indent + 1, "Rd: {}", reg_string!(*rd));
                log!(indent + 1, "SFT: {}", sft_op_string!(*sft));
                log_data!(indent + 1, "SFC", *sfc);
                log!(indent + 1, "Rs: {}", reg_string!(*rs));
            },

            Invalid{..} => { log_finln!("(invalid)"); },
        }
    }
    log!(indent + 1, "Assembly: {}", crate::decode::disassemble_jekel_style(decoded_inst));
}
