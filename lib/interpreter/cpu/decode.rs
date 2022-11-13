/* NAME//TODO
 * By: John Jekel
 *
 * TODO description
 *
*/

//DecodedInstructionType's case closely matches the ISA documentation
#[allow(non_camel_case_types)]

/* Imports */

//mod upper_nibble_1111;
//mod upper_nibble_1110;

use crate::logging::log;
use crate::logging::log_noln;
use crate::logging::log_finln;

/* Constants */

//TODO

/* Macros */

macro_rules! return_type {
    ($indent:expr, $inst_type:expr) => {
        log!($indent, "Instruction Type: ");
        if cfg!(debug_assertions) {
            match $inst_type {
                DSI6 => { log_finln!("DSI6"); }
                CALL => { log_finln!("CALL"); }
                JMPF => { log_finln!("JMPF"); }
                JMPR => { log_finln!("JMPR"); }
                FIR_MOV => { log_finln!("FIR_MOV"); }
                Fraction => { log_finln!("Fraction"); }
                INT_SET => { log_finln!("INT SET"); }
                IRQ => { log_finln!("IRQ"); }
                SECBANK => { log_finln!("SECBANK"); }
                FIQ => { log_finln!("FIQ"); }
                IRQ_NEST_MODE => { log_finln!("IRQ Nest Mode"); }
                BREAK => { log_finln!("BREAK"); }
                CALLR => { log_finln!("CALLR"); }
                DIVS => { log_finln!("DIVS"); }
                DIVQ => { log_finln!("DIVQ"); }
                EXP => { log_finln!("EXP"); }
                NOP => { log_finln!("NOP"); }
                DS_Access => { log_finln!("DS Access"); }
                FR_Access=> { log_finln!("FR Access"); }
                MUL => { log_finln!("MUL"); }
                MULS => { log_finln!("MULS"); }
                Register_BITOP_Rs => { log_finln!("Register BITOP (Rs)"); }
                Register_BITOP_offset => { log_finln!("Register BITOP (offset)"); }
                Memory_BITOP_offset => { log_finln!("Memory BITOP (offset)"); }
                Memory_BITOP_Rs => { log_finln!("Memory BITOP (Rs)"); }
                sixteen_bits_Shift => { log_finln!("16 bits Shift"); }
                RETI => { log_finln!("RETI"); }
                RETF => { log_finln!("RETF"); }
                Base_plus_Disp6 => { log_finln!("Base+Disp6"); }
                IMM6 => { log_finln!("IMM6"); }
                Branch => { log_finln!("Branch"); }
                Stack_Operation => { log_finln!("Stack Operation"); }
                DS_Indirect => { log_finln!("DS_Indirect"); }
                IMM16 => { log_finln!("IMM16"); }
                Direct16 => { log_finln!("Direct16"); }
                Direct6 => { log_finln!("Direct6"); }
                Register => { log_finln!("Register"); }

                InvalidInstructionType => { log_finln!("(invalid)"); }
            }
        }

        return $inst_type;
    }
}

/* Static Variables */

//TODO

/* Types */

pub(super) enum DecodedInstructionType {
    DSI6,
    CALL,
    JMPF,
    JMPR,
    FIR_MOV,
    Fraction,
    INT_SET,
    IRQ,
    SECBANK,
    FIQ,
    IRQ_NEST_MODE,
    BREAK,
    CALLR,
    DIVS,
    DIVQ,
    EXP,
    NOP,
    DS_Access,
    FR_Access,
    MUL,
    MULS,
    Register_BITOP_Rs,
    Register_BITOP_offset,
    Memory_BITOP_offset,
    Memory_BITOP_Rs,
    sixteen_bits_Shift,
    RETI,
    RETF,
    Base_plus_Disp6,
    IMM6,
    Branch,
    Stack_Operation,
    DS_Indirect,
    IMM16,
    Direct16,
    Direct6,
    Register,

    InvalidInstructionType,
}

/* Associated Functions and Methods */

//TODO

/* Functions */

pub(super) fn decode(inst_word: u16) -> DecodedInstructionType {
    log!(1, "CPU: Decode the instruction's type");

    log_noln!(2, "First check if the instruction is obviously bad: ");
    if (inst_word == 0xFFFF) || (inst_word == 0x0000) {//All zero or all one instructions are not valid
        log_finln!("Yep.");
        return_type!(3, DecodedInstructionType::InvalidInstructionType);
    }
    log_finln!("Nope!");

    let upper_nibble = super::upper_nibble!(inst_word);
    log!(2, "Next let's look at the upper nibble: {:#06X}", upper_nibble);
    match upper_nibble {
        0b1111 => {
            let secondary_group = super::secondary_group!(inst_word);
            log!(3, "The upper nibble is 0b1111, so let's inspect the secondary group {:#05b}", secondary_group);
            match secondary_group {
                0b000 => {
                    let rd_index = super::rd_index!(inst_word);
                    log!(4, "The secondary group is 0b000, so let's inspect Rd {:#05b}", rd_index);
                    if rd_index == 0b111 {
                        return_type!(5, DecodedInstructionType::DSI6);
                    } else {
                        let bits_54 = (inst_word >> 4) & 0b11;
                        log!(5, "Rd is not 0b111, so inspect bits [5:4] {:#04b}", bits_54);
                        match bits_54 {
                            0b00 => { return_type!(6, DecodedInstructionType::MUL); },
                            0b01 => { return_type!(6, DecodedInstructionType::InvalidInstructionType); },
                            0b10 => { return_type!(6, DecodedInstructionType::DS_Access); },
                            0b11 => { return_type!(6, DecodedInstructionType::FR_Access); },
                            _ => { panic!(); },//This should never occur
                        }
                    }
                },
                0b001 => {
                    let bit_9 = ((inst_word >> 9) & 0b1);
                    log!(4, "The secondary group is 0b001, so let's inspect bit 9 {:#03b}", bit_9);
                    if bit_9 == 0b1 {
                        return_type!(5, DecodedInstructionType::InvalidInstructionType);
                    } else {
                        return_type!(5, DecodedInstructionType::CALL);
                    }
                },
                0b010 => {
                    let rd_index = super::rd_index!(inst_word);
                    log!(4, "The secondary group is 0b010, so let's inspect Rd {:#05b}", rd_index);
                    if rd_index == 0b111 {
                        return_type!(5, DecodedInstructionType::JMPF);
                    } else {
                        return_type!(5, DecodedInstructionType::MULS);
                    }
                },
                0b011 => {
                    let rd_index = super::rd_index!(inst_word);
                    log!(4, "The secondary group is 0b011, so let's inspect Rd {:#05b}", rd_index);
                    if rd_index == 0b111 {
                        return_type!(5, DecodedInstructionType::JMPR);
                    } else {
                        return_type!(5, DecodedInstructionType::MULS);
                    }
                },
                0b100 => { return_type!(4, DecodedInstructionType::MUL); },
                0b101 => {
                    let bit_5 = ((inst_word >> 5) & 0b1);//Look at bit 5 first to split the opcode space in twoish
                    log!(4, "The secondary group is 0b101, so let's inspect bit 5 {:#03b}", bit_5);
                    if bit_5 == 0b1 {
                        unimplemented!();//TODO
                    } else {
                        unimplemented!();//TODO
                    }
                },
                0b110 | 0b111 => { return_type!(4, DecodedInstructionType::MULS); },
                _ => { panic!(); },//This should never occur
            }
        },
        0b1110 => {
            unimplemented!();//TODO
        },
        0b0101 | 0b0111 => {
            return_type!(3, DecodedInstructionType::Branch);
        },
        upper_nibble => {
            unimplemented!();//TODO
        },
    }
}
