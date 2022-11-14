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
        log_noln!($indent, "Instruction Type: ");
        if cfg!(debug_assertions) {
            match $inst_type {
                DecodedInstructionType::DSI6 => { log_finln!("DSI6"); }
                DecodedInstructionType::CALL => { log_finln!("CALL"); }
                DecodedInstructionType::JMPF => { log_finln!("JMPF"); }
                DecodedInstructionType::JMPR => { log_finln!("JMPR"); }
                DecodedInstructionType::FIR_MOV => { log_finln!("FIR_MOV"); }
                DecodedInstructionType::Fraction => { log_finln!("Fraction"); }
                DecodedInstructionType::INT_SET => { log_finln!("INT SET"); }
                DecodedInstructionType::IRQ => { log_finln!("IRQ"); }
                DecodedInstructionType::SECBANK => { log_finln!("SECBANK"); }
                DecodedInstructionType::FIQ => { log_finln!("FIQ"); }
                DecodedInstructionType::IRQ_Nest_Mode => { log_finln!("IRQ Nest Mode"); }
                DecodedInstructionType::BREAK => { log_finln!("BREAK"); }
                DecodedInstructionType::CALLR => { log_finln!("CALLR"); }
                DecodedInstructionType::DIVS => { log_finln!("DIVS"); }
                DecodedInstructionType::DIVQ => { log_finln!("DIVQ"); }
                DecodedInstructionType::EXP => { log_finln!("EXP"); }
                DecodedInstructionType::NOP => { log_finln!("NOP"); }
                DecodedInstructionType::DS_Access => { log_finln!("DS Access"); }
                DecodedInstructionType::FR_Access=> { log_finln!("FR Access"); }
                DecodedInstructionType::MUL => { log_finln!("MUL"); }
                DecodedInstructionType::MULS => { log_finln!("MULS"); }
                DecodedInstructionType::Register_BITOP_Rs => { log_finln!("Register BITOP (Rs)"); }
                DecodedInstructionType::Register_BITOP_offset => { log_finln!("Register BITOP (offset)"); }
                DecodedInstructionType::Memory_BITOP_offset => { log_finln!("Memory BITOP (offset)"); }
                DecodedInstructionType::Memory_BITOP_Rs => { log_finln!("Memory BITOP (Rs)"); }
                DecodedInstructionType::sixteen_bits_Shift => { log_finln!("16 bits Shift"); }
                DecodedInstructionType::RETI => { log_finln!("RETI"); }
                DecodedInstructionType::RETF => { log_finln!("RETF"); }
                DecodedInstructionType::Base_plus_Disp6 => { log_finln!("Base+Disp6"); }
                DecodedInstructionType::IMM6 => { log_finln!("IMM6"); }
                DecodedInstructionType::Branch => { log_finln!("Branch"); }
                DecodedInstructionType::Stack_Operation => { log_finln!("Stack Operation"); }
                DecodedInstructionType::DS_Indirect => { log_finln!("DS_Indirect"); }
                DecodedInstructionType::IMM16 => { log_finln!("IMM16"); }
                DecodedInstructionType::Direct16 => { log_finln!("Direct16"); }
                DecodedInstructionType::Direct6 => { log_finln!("Direct6"); }
                DecodedInstructionType::Register => { log_finln!("Register"); }

                DecodedInstructionType::InvalidInstructionType => { log_finln!("(invalid)"); }
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
    IRQ_Nest_Mode,
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
            log!(3, "The upper nibble is 0b1111, so let's inspect the secondary group: {:#05b}", secondary_group);
            match secondary_group {
                0b000 => {
                    let rd_index = super::rd_index!(inst_word);
                    log!(4, "The secondary group is 0b000, so let's inspect Rd: {:#05b}", rd_index);
                    if rd_index == 0b111 {
                        return_type!(5, DecodedInstructionType::DSI6);
                    } else {
                        let bits_54 = (inst_word >> 4) & 0b11;
                        log!(5, "Rd is not 0b111, so inspect bits [5:4]: {:#04b}", bits_54);
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
                    let bit_9 = (inst_word >> 9) & 0b1;
                    log!(4, "The secondary group is 0b001, so let's inspect bit 9: {:#03b}", bit_9);
                    if bit_9 == 0b1 {
                        return_type!(5, DecodedInstructionType::InvalidInstructionType);
                    } else {
                        return_type!(5, DecodedInstructionType::CALL);
                    }
                },
                0b010 => {
                    let rd_index = super::rd_index!(inst_word);
                    log!(4, "The secondary group is 0b010, so let's inspect Rd: {:#05b}", rd_index);
                    if rd_index == 0b111 {
                        return_type!(5, DecodedInstructionType::JMPF);
                    } else {
                        return_type!(5, DecodedInstructionType::MULS);
                    }
                },
                0b011 => {
                    let rd_index = super::rd_index!(inst_word);
                    log!(4, "The secondary group is 0b011, so let's inspect Rd: {:#05b}", rd_index);
                    if rd_index == 0b111 {
                        return_type!(5, DecodedInstructionType::JMPR);
                    } else {
                        return_type!(5, DecodedInstructionType::MULS);
                    }
                },
                0b100 => { return_type!(4, DecodedInstructionType::MUL); },
                0b101 => {
                    let bit_5 = (inst_word >> 5) & 0b1;//Look at bit 5 first to split the opcode space in twoish
                    log!(4, "The secondary group is 0b101, so let's inspect bit 5: {:#03b}", bit_5);
                    if bit_5 == 0b1 {
                        let bits_210 = inst_word & 0b111;//Look at the lowest 3 bits to decide what it is
                        log!(5, "Bit 5 is set, so let's inspect the lowest 3 bits: {:#05b}", bits_210);
                        match inst_word & 0b111 {
                            0b000 => { return_type!(6, DecodedInstructionType::BREAK); },
                            0b001 => { return_type!(6, DecodedInstructionType::CALLR); },
                            0b010 => { return_type!(6, DecodedInstructionType::DIVS); },
                            0b011 => { return_type!(6, DecodedInstructionType::DIVQ); },
                            0b100 => { return_type!(6, DecodedInstructionType::EXP); },
                            0b101 => { return_type!(6, DecodedInstructionType::NOP); },
                            _ => { return_type!(6, DecodedInstructionType::InvalidInstructionType); },
                        }
                    } else {
                        let bits_432 = (inst_word >> 2) & 0b111;//Look at bits 4:2 to split things further
                        log!(5, "Bit 5 is not set, so let's inspect the bits [4:2]: {:#05b}", bits_432);
                        match bits_432 {
                            0b000 => { return_type!(6, DecodedInstructionType::INT_SET); },
                            0b001 => {
                                let bit_1 = (inst_word >> 1) & 0b1;
                                log!(6, "Bits [4:2] are 0b001, so let's inspect bit 1: {:#03b}", bit_1);
                                if bit_1 == 0b1 {
                                    return_type!(7, DecodedInstructionType::Fraction);
                                } else {
                                    return_type!(7, DecodedInstructionType::FIR_MOV);
                                }
                            },
                            0b010 => {
                                let bit_1 = (inst_word >> 1) & 0b1;
                                log!(6, "Bits [4:2] are 0b010, so let's inspect bit 1: {:#03b}", bit_1);
                                if bit_1 == 0b1 {
                                    return_type!(7, DecodedInstructionType::SECBANK);
                                } else {
                                    return_type!(7, DecodedInstructionType::IRQ);
                                }
                            },
                            0b011 => {
                                let bit_0 = inst_word & 0b1;
                                log!(6, "Bits [4:2] are 0b011, so let's inspect bit 0: {:#03b}", bit_0);
                                if bit_0 == 0b1 {
                                    return_type!(7, DecodedInstructionType::IRQ_Nest_Mode);
                                } else {
                                    return_type!(7, DecodedInstructionType::FIQ);
                                }
                            },
                            _ => { return_type!(6, DecodedInstructionType::InvalidInstructionType); },
                        }
                    }
                },
                0b110 | 0b111 => { return_type!(4, DecodedInstructionType::MULS); },
                _ => { panic!(); },//This should never occur
            }
        },
        0b1110 => {
            log!(3, "The upper nibble is 0b1110, so let's check if this is a branch");

            let secondary_group = super::secondary_group!(inst_word);
            if (super::rd_index!(inst_word) == 0b111) && ((secondary_group == 0b000) || (secondary_group == 0b001)) {
                return_type!(4, DecodedInstructionType::Branch);
            } else {
                log!(4, "This isn't a branch, so let's inspect the secondary group: {:#05b}", secondary_group);
                match secondary_group {
                    0b000 => {
                        let bit_3 = (inst_word >> 3) & 0b1;
                        log!(5, "The secondary group is 0b000, so let's inspect bit 3: {:#03b}", bit_3);
                        if bit_3 == 0b1 {
                            return_type!(6, DecodedInstructionType::MUL);
                        } else {
                            return_type!(6, DecodedInstructionType::Register_BITOP_Rs);
                        }
                    },
                    0b001 => { return_type!(5, DecodedInstructionType::Register_BITOP_offset); },
                    0b010 => { return_type!(5, DecodedInstructionType::MULS); },
                    0b011 => { return_type!(5, DecodedInstructionType::InvalidInstructionType); },
                    0b100 | 0b101 => {
                        let bit_3 = (inst_word >> 3) & 0b1;
                        log!(5, "The secondary group is 0b000, so let's inspect bit 3: {:#03b}", bit_3);
                        if bit_3 == 0b1 {
                            return_type!(6, DecodedInstructionType::sixteen_bits_Shift);
                        } else {
                            return_type!(6, DecodedInstructionType::Memory_BITOP_Rs);
                        }
                    },
                    0b110 | 0b111 => { return_type!(5, DecodedInstructionType::Memory_BITOP_offset); }
                    _ => { panic!(); },//This should never occur
                }
            }
        },
        0b0101 | 0b0111 => {
            log!(3, "The upper nibble indicates this is likely a branch, verifying that it is valid...");
            let secondary_group = super::secondary_group!(inst_word);
            if (super::rd_index!(inst_word) == 0b111) && ((secondary_group == 0b000) || (secondary_group == 0b001)) {
                return_type!(4, DecodedInstructionType::Branch);
            } else {
                return_type!(4, DecodedInstructionType::InvalidInstructionType);
            }
        },
        upper_nibble => {
            unimplemented!();//TODO
        },
    }
}
