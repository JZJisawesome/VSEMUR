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

//TODO (also pub(crate) use the_macro statements here too)

/* Static Variables */

//TODO

/* Types */

//TODO

/* Associated Functions and Methods */

//TODO

/* Functions */

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


macro_rules! log_data {
    ($indent:expr, $pretext:expr, $data:expr) => {
        log!($indent, "{}: {1:#04X} | {1:#08b} | unsigned {1}", $pretext, $data);
    };
}
pub(crate) use log_data;//FIXME prevent having to export these helper macros to the whole crate (limitation of rust)

macro_rules! log_addr {
    ($indent:expr, $pretext:expr, $addr:expr) => {
        log!($indent, "{}: {:#04X}_{:04X}", $pretext, $addr >> 16, $addr + 0xFFFF);
    };
}
pub(crate) use log_addr;//FIXME prevent having to export these helper macros to the whole crate (limitation of rust)

macro_rules! log_inst {
    ($indent:expr, $decoded_instruction: expr) => {
        use crate::logging::log;
        use crate::logging::log_noln;
        use crate::logging::log_finln;

        use crate::interpreter::cpu::decode::instruction_printing::log_data;
        use crate::interpreter::cpu::decode::instruction_printing::log_addr;
        use crate::interpreter::cpu::decode::instruction_printing::get_inst_assembly;

        use crate::interpreter::cpu::decode::DecodedInstruction::*;

        log_noln!($indent, "Instruction: ");
        if cfg!(debug_assertions) {//TODO print sub fields of each type too (on new lines indented under it)
            match $decoded_instruction {
                DSI6{imm6} => {
                    log_finln!("DSI6");
                    log_data!($indent + 1, "imm6", imm6);
                },
                CALL{a22} => {
                    log_finln!("CALL");
                    log_addr!($indent + 1, "a22", a22);
                },
                JMPF{..} => {
                    log_finln!("JMPF");
                },
                JMPR{..} => { log_finln!("JMPR"); },
                FIR_MOV{..}=> { log_finln!("FIR_MOV"); },
                Fraction{..} => { log_finln!("Fraction"); },
                INT_SET{..} => { log_finln!("INT SET"); },
                IRQ{..} => { log_finln!("IRQ"); },
                SECBANK{..} => { log_finln!("SECBANK"); },
                FIQ{..} => { log_finln!("FIQ"); },
                IRQ_Nest_Mode{..} => { log_finln!("IRQ Nest Mode"); },
                BREAK{..} => { log_finln!("BREAK"); },
                CALLR{..} => { log_finln!("CALLR"); },
                DIVS{..} => { log_finln!("DIVS"); },
                DIVQ{..} => { log_finln!("DIVQ"); },
                EXP{..} => { log_finln!("EXP"); },
                NOP => { log_finln!("NOP"); },
                DS_Access{..} => { log_finln!("DS Access"); },
                FR_Access{..} => { log_finln!("FR Access"); },
                MUL{..} => { log_finln!("MUL"); },
                MULS{..} => { log_finln!("MULS"); },
                Register_BITOP_Rs{..} => { log_finln!("Register BITOP (Rs)"); },
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
        log!($indent + 1, "Assembly: {}", get_inst_assembly!($decoded_instruction));
    };
}
pub(crate) use log_inst;//FIXME prevent having to export these helper macros to the whole crate (limitation of rust)
