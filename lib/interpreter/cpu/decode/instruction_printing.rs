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

//TODO move to a seperate disassembly module later
macro_rules! get_inst_assembly {
    ($decoded_instruction: expr) => {{
        let assembly: &str;
        {
            use crate::interpreter::cpu::decode::DecodedInstruction::*;
            assembly = "TODO";//TODO match based on $decoded_instruction and format like assembly
        }
        assembly
    }}
}
pub(crate) use get_inst_assembly;//FIXME prevent having to export these helper macros to the whole crate (limitation of rust)

macro_rules! log_inst {
    ($indent:expr, $decoded_instruction: expr) => {
        //Compile times were getting a bit too long due to the large macro inlining in decode
        if cfg!(debug_assertions) {
            crate::interpreter::cpu::decode::instruction_printing::log_inst_func($indent, $decoded_instruction);
        }
    };
}
pub(crate) use log_inst;//FIXME prevent having to export these helper macros to the whole crate (limitation of rust)

macro_rules! log_data {
    ($indent:expr, $pretext:expr, $data:expr) => {
        log!($indent, "{}: {1:#04X} | {1:#08b} | unsigned {1}", $pretext, $data);
    };
}
pub(crate) use log_data;//FIXME prevent having to export these helper macros to the whole crate (limitation of rust)

macro_rules! log_addr {
    ($indent:expr, $pretext:expr, $addr:expr) => {
        log!($indent, "{}: {:#04X}_{:04X}", $pretext, $addr >> 16, $addr & 0xFFFF);
    };
}
pub(crate) use log_addr;//FIXME prevent having to export these helper macros to the whole crate (limitation of rust)

macro_rules! reg_string {
    ($reg:expr) => {{
        let string: &str;
        {
            use crate::interpreter::cpu::decode::DecodedRegister::*;
            match $reg  {
                SP => { string = "SP"; },
                R1_SR1 => { string = "R1"; },
                R2_SR2 => { string = "R2"; },
                R3_SR3 => { string = "R3"; },
                R4_SR4 => { string = "R4"; },
                BP => { string = "BP"; },
                SR => { string = "SR"; },
                PC => { string = "PC"; },

                Invalid => { string = "(invalid)"; }
            }
        }
        string
    }};
}
pub(crate) use reg_string;//FIXME prevent having to export these helper macros to the whole crate (limitation of rust)

//TODO (also pub(crate) use the_macro statements here too)

/* Static Variables */

//TODO

/* Types */

//TODO

/* Associated Functions and Methods */

//TODO

/* Functions */

pub(super) fn log_inst_func(indent: u8, decoded_inst: &crate::interpreter::cpu::decode::DecodedInstruction) {
    use crate::logging::log;
    use crate::logging::log_noln;
    use crate::logging::log_finln;

    use crate::interpreter::cpu::decode::DecodedInstruction::*;

    log_noln!(indent, "Instruction: ");
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
                log!(indent + 1, "Size: {}", *size);
                log!(indent + 1, "Rs: {}", reg_string!(*rs));
            },
            Register_BITOP_Rs{..} => {//TODO this and the rest
                log_finln!("Register BITOP (Rs)");
            },
            Register_BITOP_offset{..} => { log_finln!("Register BITOP (offset)"); },
            Memory_BITOP_offset{..} => { log_finln!("Memory BITOP (offset)"); },
            Memory_BITOP_Rs{..} => { log_finln!("Memory BITOP (Rs)"); },
            sixteen_bits_Shift{..} => { log_finln!("16 bits Shift"); },
            RETI{..} => { log_finln!("RETI"); },
            RETF{..} => { log_finln!("RETF"); },
            Base_plus_Disp6{..} => { log_finln!("Base+Disp6"); },
            IMM6{..} => { log_finln!("IMM6"); },
            Branch{..} => { log_finln!("Branch"); },
            Stack_Operation{..} => { log_finln!("Stack Operation"); },
            DS_Indirect{..} => { log_finln!("DS_Indirect"); },
            IMM16{..} => { log_finln!("IMM16"); },
            Direct16{..} => { log_finln!("Direct16"); },
            Direct6{..} => { log_finln!("Direct6"); },
            Register{..} => { log_finln!("Register"); },

            Invalid{..} => { log_finln!("(invalid)"); },
        }
    }
    log!(indent + 1, "Assembly: {}", get_inst_assembly!(decoded_instruction));
}
