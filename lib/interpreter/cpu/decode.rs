/* NAME//TODO
 * By: John Jekel
 *
 * TODO description
 *
*/

//DecodedInstruction's case closely matches the ISA documentation
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

macro_rules! return_inst {
    ($indent:expr, $decoded_inst_out:expr, $inst_type:expr) => {
        log_noln!($indent, "Instruction: ");
        if cfg!(debug_assertions) {
            match $inst_type {
                DecodedInstruction::DSI6{..} => { log_finln!("DSI6"); }
                DecodedInstruction::CALL{..} => { log_finln!("CALL"); }
                DecodedInstruction::JMPF{..} => { log_finln!("JMPF"); }
                DecodedInstruction::JMPR{..} => { log_finln!("JMPR"); }
                DecodedInstruction::FIR_MOV{..}=> { log_finln!("FIR_MOV"); }
                DecodedInstruction::Fraction{..} => { log_finln!("Fraction"); }
                DecodedInstruction::INT_SET{..} => { log_finln!("INT SET"); }
                DecodedInstruction::IRQ{..} => { log_finln!("IRQ"); }
                DecodedInstruction::SECBANK{..} => { log_finln!("SECBANK"); }
                DecodedInstruction::FIQ{..} => { log_finln!("FIQ"); }
                DecodedInstruction::IRQ_Nest_Mode{..} => { log_finln!("IRQ Nest Mode"); }
                DecodedInstruction::BREAK{..} => { log_finln!("BREAK"); }
                DecodedInstruction::CALLR{..} => { log_finln!("CALLR"); }
                DecodedInstruction::DIVS{..} => { log_finln!("DIVS"); }
                DecodedInstruction::DIVQ{..} => { log_finln!("DIVQ"); }
                DecodedInstruction::EXP{..} => { log_finln!("EXP"); }
                DecodedInstruction::NOP{..} => { log_finln!("NOP"); }
                DecodedInstruction::DS_Access{..} => { log_finln!("DS Access"); }
                DecodedInstruction::FR_Access{..} => { log_finln!("FR Access"); }
                DecodedInstruction::MUL{..} => { log_finln!("MUL"); }
                DecodedInstruction::MULS{..} => { log_finln!("MULS"); }
                DecodedInstruction::Register_BITOP_Rs{..} => { log_finln!("Register BITOP (Rs)"); }
                DecodedInstruction::Register_BITOP_offset{..} => { log_finln!("Register BITOP (offset)"); }
                DecodedInstruction::Memory_BITOP_offset{..} => { log_finln!("Memory BITOP (offset)"); }
                DecodedInstruction::Memory_BITOP_Rs{..} => { log_finln!("Memory BITOP (Rs)"); }
                DecodedInstruction::sixteen_bits_Shift{..} => { log_finln!("16 bits Shift"); }
                DecodedInstruction::RETI{..} => { log_finln!("RETI"); }
                DecodedInstruction::RETF{..} => { log_finln!("RETF"); }
                DecodedInstruction::Base_plus_Disp6{..} => { log_finln!("Base+Disp6"); }
                DecodedInstruction::IMM6{..} => { log_finln!("IMM6"); }
                DecodedInstruction::Branch{..} => { log_finln!("Branch"); }
                DecodedInstruction::Stack_Operation{..} => { log_finln!("Stack Operation"); }
                DecodedInstruction::DS_Indirect{..} => { log_finln!("DS_Indirect"); }
                DecodedInstruction::IMM16{..} => { log_finln!("IMM16"); }
                DecodedInstruction::Direct16{..} => { log_finln!("Direct16"); }
                DecodedInstruction::Direct6{..} => { log_finln!("Direct6"); }
                DecodedInstruction::Register{..} => { log_finln!("Register"); }

                DecodedInstruction::InvalidInstructionType{..} => { log_finln!("(invalid)"); }
            }
        }

        *$decoded_inst_out = $inst_type;
        return;
    }
}

macro_rules! rd_index {
    ($inst_word:expr) => {
        (($inst_word >> 9) & 0b111) as u8
    };
}

macro_rules! rs_index {
    ($inst_word:expr) => {
        ($inst_word & 0b111) as u8
    };
}

macro_rules! imm6 {
    ($inst_word:expr) => {
        ($inst_word & 0b111111) as u8
    };
}

macro_rules! upper_nibble {
    ($inst_word:expr) => {
        $inst_word >> 12
    };
}

macro_rules! secondary_group {
    ($inst_word:expr) => {
        ($inst_word >> 6) & 0b111
    };
}

/* Static Variables */

//TODO

/* Types */

pub(super) enum DecodedInstruction {
    DSI6{imm6: u8},
    CALL/*{a22_top6: u8}*/,//Execute needs to get wg2//TODO for the cached interpreter get everything in decode
    JMPF/*{a22_top6: u8}*/,//Execute needs to get wg2//TODO for the cached interpreter get everything in decode
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
    IMM16,//Execute needs to get wg2
    Direct16,//Execute needs to get wg2
    Direct6,
    Register,

    InvalidInstructionType,
}

/* Associated Functions and Methods */

//TODO

/* Functions */

pub(super) fn decode(inst_word: u16, decoded_inst: &mut DecodedInstruction) {
    log!(1, "CPU: Decode instruction");

    log_noln!(2, "First check if the instruction is obviously bad: ");
    if (inst_word == 0xFFFF) || (inst_word == 0x0000) {//All zero or all one instructions are not valid
        log_finln!("Yep.");
        return_inst!(3, decoded_inst, DecodedInstruction::InvalidInstructionType);
    }
    log_finln!("Nope!");

    let upper_nibble = upper_nibble!(inst_word);
    log!(2, "Next let's look at the upper nibble: {:#06X}", upper_nibble);
    match upper_nibble {
        0b1111 => {
            let secondary_group = secondary_group!(inst_word);
            log!(3, "The upper nibble is 0b1111, so let's inspect the secondary group: {:#05b}", secondary_group);
            match secondary_group {
                0b000 => {
                    let rd_index = rd_index!(inst_word);
                    log!(4, "The secondary group is 0b000, so let's inspect Rd: {:#05b}", rd_index);
                    if rd_index == 0b111 {
                        return_inst!(5, decoded_inst, DecodedInstruction::DSI6{imm6: imm6!(inst_word)});
                    } else {
                        let bits_54 = (inst_word >> 4) & 0b11;
                        log!(5, "Rd is not 0b111, so inspect bits [5:4]: {:#04b}", bits_54);
                        match bits_54 {
                            0b00 => { return_inst!(6, decoded_inst, DecodedInstruction::MUL); },
                            0b01 => { return_inst!(6, decoded_inst, DecodedInstruction::InvalidInstructionType); },
                            0b10 => { return_inst!(6, decoded_inst, DecodedInstruction::DS_Access); },
                            0b11 => { return_inst!(6, decoded_inst, DecodedInstruction::FR_Access); },
                            _ => { panic!(); },//This should never occur
                        }
                    }
                },
                0b001 => {
                    let bit_9 = (inst_word >> 9) & 0b1;
                    log!(4, "The secondary group is 0b001, so let's inspect bit 9: {:#03b}", bit_9);
                    if bit_9 == 0b1 {
                        return_inst!(5, decoded_inst, DecodedInstruction::InvalidInstructionType);
                    } else {
                        return_inst!(5, decoded_inst, DecodedInstruction::CALL);
                    }
                },
                0b010 => {
                    let rd_index = rd_index!(inst_word);
                    log!(4, "The secondary group is 0b010, so let's inspect Rd: {:#05b}", rd_index);
                    if rd_index == 0b111 {
                        return_inst!(5, decoded_inst, DecodedInstruction::JMPF);
                    } else {
                        return_inst!(5, decoded_inst, DecodedInstruction::MULS);
                    }
                },
                0b011 => {
                    let rd_index = rd_index!(inst_word);
                    log!(4, "The secondary group is 0b011, so let's inspect Rd: {:#05b}", rd_index);
                    if rd_index == 0b111 {
                        return_inst!(5, decoded_inst, DecodedInstruction::JMPR);
                    } else {
                        return_inst!(5, decoded_inst, DecodedInstruction::MULS);
                    }
                },
                0b100 => { return_inst!(4, decoded_inst, DecodedInstruction::MUL); },
                0b101 => {
                    let bit_5 = (inst_word >> 5) & 0b1;//Look at bit 5 first to split the opcode space in twoish
                    log!(4, "The secondary group is 0b101, so let's inspect bit 5: {:#03b}", bit_5);
                    if bit_5 == 0b1 {
                        let bits_210 = inst_word & 0b111;//Look at the lowest 3 bits to decide what it is
                        log!(5, "Bit 5 is set, so let's inspect the lowest 3 bits: {:#05b}", bits_210);
                        match inst_word & 0b111 {
                            0b000 => { return_inst!(6, decoded_inst, DecodedInstruction::BREAK); },
                            0b001 => { return_inst!(6, decoded_inst, DecodedInstruction::CALLR); },
                            0b010 => { return_inst!(6, decoded_inst, DecodedInstruction::DIVS); },
                            0b011 => { return_inst!(6, decoded_inst, DecodedInstruction::DIVQ); },
                            0b100 => { return_inst!(6, decoded_inst, DecodedInstruction::EXP); },
                            0b101 => { return_inst!(6, decoded_inst, DecodedInstruction::NOP); },
                            _ => { return_inst!(6, decoded_inst, DecodedInstruction::InvalidInstructionType); },
                        }
                    } else {
                        let bits_432 = (inst_word >> 2) & 0b111;//Look at bits 4:2 to split things further
                        log!(5, "Bit 5 is not set, so let's inspect the bits [4:2]: {:#05b}", bits_432);
                        match bits_432 {
                            0b000 => { return_inst!(6, decoded_inst, DecodedInstruction::INT_SET); },
                            0b001 => {
                                let bit_1 = (inst_word >> 1) & 0b1;
                                log!(6, "Bits [4:2] are 0b001, so let's inspect bit 1: {:#03b}", bit_1);
                                if bit_1 == 0b1 {
                                    return_inst!(7, decoded_inst, DecodedInstruction::Fraction);
                                } else {
                                    return_inst!(7, decoded_inst, DecodedInstruction::FIR_MOV);
                                }
                            },
                            0b010 => {
                                let bit_1 = (inst_word >> 1) & 0b1;
                                log!(6, "Bits [4:2] are 0b010, so let's inspect bit 1: {:#03b}", bit_1);
                                if bit_1 == 0b1 {
                                    return_inst!(7, decoded_inst, DecodedInstruction::SECBANK);
                                } else {
                                    return_inst!(7, decoded_inst, DecodedInstruction::IRQ);
                                }
                            },
                            0b011 => {
                                let bit_0 = inst_word & 0b1;
                                log!(6, "Bits [4:2] are 0b011, so let's inspect bit 0: {:#03b}", bit_0);
                                if bit_0 == 0b1 {
                                    return_inst!(7, decoded_inst, DecodedInstruction::IRQ_Nest_Mode);
                                } else {
                                    return_inst!(7, decoded_inst, DecodedInstruction::FIQ);
                                }
                            },
                            _ => { return_inst!(6, decoded_inst, DecodedInstruction::InvalidInstructionType); },
                        }
                    }
                },
                0b110 | 0b111 => { return_inst!(4, decoded_inst, DecodedInstruction::MULS); },
                _ => { panic!(); },//This should never occur
            }
        },
        0b1110 => {
            log!(3, "The upper nibble is 0b1110, so let's check if this is a branch");

            let secondary_group = secondary_group!(inst_word);
            if (rd_index!(inst_word) == 0b111) && ((secondary_group == 0b000) || (secondary_group == 0b001)) {
                return_inst!(4, decoded_inst, DecodedInstruction::Branch);
            } else {
                log!(4, "This isn't a branch, so let's inspect the secondary group: {:#05b}", secondary_group);
                match secondary_group {
                    0b000 => {
                        let bit_3 = (inst_word >> 3) & 0b1;
                        log!(5, "The secondary group is 0b000, so let's inspect bit 3: {:#03b}", bit_3);
                        if bit_3 == 0b1 {
                            return_inst!(6, decoded_inst, DecodedInstruction::MUL);
                        } else {
                            return_inst!(6, decoded_inst, DecodedInstruction::Register_BITOP_Rs);
                        }
                    },
                    0b001 => { return_inst!(5, decoded_inst, DecodedInstruction::Register_BITOP_offset); },
                    0b010 => { return_inst!(5, decoded_inst, DecodedInstruction::MULS); },
                    0b011 => { return_inst!(5, decoded_inst, DecodedInstruction::InvalidInstructionType); },
                    0b100 | 0b101 => {
                        let bit_3 = (inst_word >> 3) & 0b1;
                        log!(5, "The secondary group is 0b000, so let's inspect bit 3: {:#03b}", bit_3);
                        if bit_3 == 0b1 {
                            return_inst!(6, decoded_inst, DecodedInstruction::sixteen_bits_Shift);
                        } else {
                            return_inst!(6, decoded_inst, DecodedInstruction::Memory_BITOP_Rs);
                        }
                    },
                    0b110 | 0b111 => { return_inst!(5, decoded_inst, DecodedInstruction::Memory_BITOP_offset); }
                    _ => { panic!(); },//This should never occur
                }
            }
        },
        0b0101 | 0b0111 => {
            log!(3, "The upper nibble indicates this is likely a branch, verifying that it is valid...");
            let secondary_group = secondary_group!(inst_word);
            if (rd_index!(inst_word) == 0b111) && ((secondary_group == 0b000) || (secondary_group == 0b001)) {
                return_inst!(4, decoded_inst, DecodedInstruction::Branch);
            } else {
                return_inst!(4, decoded_inst, DecodedInstruction::InvalidInstructionType);
            }
        },
        upper_nibble => {
            log!(3, "The upper nibble is 0b1110, so let's check if this is a branch");

            let secondary_group = secondary_group!(inst_word);
            if (rd_index!(inst_word) == 0b111) && ((secondary_group == 0b000) || (secondary_group == 0b001)) {
                return_inst!(4, decoded_inst, DecodedInstruction::Branch);
            } else {
                log!(4, "This isn't a branch, so let's inspect the secondary group: {:#05b}", secondary_group);
                match secondary_group {
                    0b000 => { return_inst!(5, decoded_inst, DecodedInstruction::Base_plus_Disp6); },
                    0b001 => { return_inst!(5, decoded_inst, DecodedInstruction::IMM6); },
                    0b010 => {
                        unimplemented!();//TODO RETI and RETF
                    },
                    0b011 => { return_inst!(5, decoded_inst, DecodedInstruction::DS_Indirect); },
                    0b100 => {
                        let bits_543 = (inst_word >> 3) & 0b111;
                        log!(5, "The secondary group is 0b100, so let's inspect bits [5:3]: {:#03b}", bits_543);
                        match bits_543 {
                            0b001 => { return_inst!(6, decoded_inst, DecodedInstruction::IMM16); },
                            0b010 | 0b011 => { return_inst!(6, decoded_inst, DecodedInstruction::Direct16); },
                            _ => { return_inst!(6, decoded_inst, DecodedInstruction::Register); },
                        }
                    },
                    0b101 | 0b110 => { return_inst!(6, decoded_inst, DecodedInstruction::Register); },
                    0b111 => {
                        unimplemented!();//TODO what about Direct6 and Register conflict?
                    },
                    _ => { panic!(); },//This should never occur
                }
            }
        },
    }
}
