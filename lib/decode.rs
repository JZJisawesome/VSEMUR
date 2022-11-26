/* decode.rs
 * Copyright (C) 2022 John Jekel
 * See the LICENSE file at the root of the project for licensing info.
 *
 * Decodes unSP instructions. See README.md for how this is done (it is black magic).
 *
*/

//!VSEMUR Decode and Disassembler Logic
//!
//!By: John Jekel
//!
//!Decodes and disassembles unSP instructions.
//!
//!See README.md for how this is done (it is black magic).
//!
//!# Example usage
//!
//!```
//!//TODO
//!```

//TODO remove this once everything is implemented
#![allow(unused_macros)]

/* Imports */

mod instruction_printing;
mod disassemble;
mod common;

pub use disassemble::disassemble_jekel_style;//Style for decode functions (logging), and for vsemur-disassemble
pub use disassemble::disassemble_generalplus_style;//Style for vsemur-disassemble
pub use disassemble::disassemble_mame_style;//Style for vsemur-disassemble

use crate::debug_panic;
use crate::logging::log;
use crate::logging::log_noln;
use crate::logging::log_finln;

use DecodedInstruction::*;

/* Constants */

//TODO

/* Macros */

macro_rules! return_inst {
    ($indent:expr, $decoded_inst_out:expr, $inst_type:expr) => {
        instruction_printing::log_inst!($indent, &$inst_type);
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

#[allow(non_camel_case_types)]
#[derive(Copy, Clone)]
pub enum DecodedInstruction {
    DSI6{imm6: u8},
    CALL{a22: u32},//Lower 16 bits are retrived in decode_wg2
    JMPF{a22: u32},//Lower 16 bits are retrived in decode_wg2
    JMPR,
    FIR_MOV{fir: bool},
    Fraction{fra: bool},
    INT_SET{f: bool, i: bool},
    IRQ{i: bool},
    SECBANK{s: bool},
    FIQ{f: bool},
    IRQ_Nest_Mode{n: bool},
    BREAK,
    CALLR,
    DIVS,
    DIVQ,
    EXP,
    NOP,
    DS_Access{w: bool, rs: DecodedRegister},
    FR_Access{w: bool, rs: DecodedRegister},
    MUL{s_rs: bool, rd: DecodedRegister, s_rd: bool, rs: DecodedRegister},
    MULS{s_rs: bool, rd: DecodedRegister, s_rd: bool, size: u8, rs: DecodedRegister},
    Register_BITOP_Rs{rd: DecodedRegister, op: DecodedBitOp, rs: DecodedRegister},
    Register_BITOP_offset{rd: DecodedRegister, op: DecodedBitOp, offset: u8},
    Memory_BITOP_offset{rd: DecodedRegister, d: bool, op: DecodedBitOp, offset: u8},
    Memory_BITOP_Rs{rd: DecodedRegister, d: bool, op: DecodedBitOp, rs: DecodedRegister},
    sixteen_bits_Shift{rd: DecodedRegister, op: DecodedLSFTOp, rs: DecodedRegister},
    RETI,
    RETF,
    Base_plus_Disp6{op: DecodedALUOp, rd: DecodedRegister, imm6: u8},
    IMM6{op: DecodedALUOp, rd: DecodedRegister, imm6: u8},
    Branch{op: DecodedBranchOp, d: bool, imm6: u8},
    Stack_Operation{op: DecodedStackOp, rd_index: u8, size: u8, rs: DecodedRegister},//Providing rd_index instead of rd since it allow for one to just start incrementing/decrementing it right away
    DS_Indirect{op: DecodedALUOp, rd: DecodedRegister, d: bool, at: DecodedAtOp, rs: DecodedRegister},
    IMM16{op: DecodedALUOp, rd: DecodedRegister, rs: DecodedRegister, imm16: u16},//imm16 is retrived in decode_wg2
    Direct16{op: DecodedALUOp, rd: DecodedRegister, w: bool, rs: DecodedRegister, a16: u16},//a16 is retrived in decode_wg2
    Direct6{op: DecodedALUOp, rd: DecodedRegister, a6: u8},
    Register{op: DecodedALUOp, rd: DecodedRegister, sft: DecodedSFTOp, sfc: u8, rs: DecodedRegister},

    Invalid,
}

#[derive(Copy, Clone)]
pub enum DecodedALUOp {
    ADD,
    ADC,
    SUB,
    SBC,
    CMP,
    NEG,
    XOR,
    LOAD,
    OR,
    AND,
    TEST,
    STORE,

    Invalid,
}

#[allow(non_camel_case_types)]
#[derive(Copy, Clone)]
pub enum DecodedBranchOp {
    JCC_JB_JNAE,
    JCS_JNB_JAE,
    JSC_JGE_JNL,
    JSS_JNGE_JL,
    JNE_JNZ,
    JZ_JE,
    JPL,
    JMI,
    JBE_JNA,
    JNBE_JA,
    JLE_JNG,
    JNLE_JG,
    JVC,
    JVS,
    JMP,

    Invalid,
}

#[derive(Copy, Clone)]
pub enum DecodedStackOp {
    PUSH,
    POP,

    Invalid,
}

#[derive(Copy, Clone)]
pub enum DecodedAtOp {
    NOP,
    PostDecrement,
    PostIncrement,
    PreIncrement,

    Invalid,
}

#[derive(Copy, Clone)]
pub enum DecodedBitOp {
    TSTB,
    SETB,
    CLRB,
    INVB,

    Invalid,
}

#[derive(Copy, Clone)]
pub enum DecodedLSFTOp {
    ASR,
    ASROR,
    LSL,
    LSLOR,
    LSR,
    LSROR,
    ROL,
    ROR,

    Invalid,
}

#[derive(Copy, Clone)]
pub enum DecodedSFTOp {
    NOP,
    ASR,
    LSL,
    LSR,
    ROL,
    ROR,

    Invalid,
}

#[allow(non_camel_case_types)]
#[derive(Copy, Clone)]
pub enum DecodedRegister {
    SP,
    R1_SR1,
    R2_SR2,
    R3_SR3,
    R4_SR4,
    BP,
    SR,
    PC,

    Invalid,
}

/* Associated Functions and Methods */

//TODO

/* Functions */

pub fn decode_wg1(inst_word: u16, decoded_inst: &mut DecodedInstruction) {
    log!(1, "unSP: Decode instruction word group 1");

    log_noln!(2, "First check if the instruction is obviously bad: ");
    if (inst_word == 0xFFFF) || (inst_word == 0x0000) {//All zero or all one instructions are not valid
        log_finln!("Yep.");
        return_inst!(3, decoded_inst, Invalid);
    }
    log_finln!("Nope!");

    let upper_nibble = upper_nibble!(inst_word);
    log!(2, "Next let's look at the upper nibble: {:#03X}", upper_nibble);
    match upper_nibble {
        0b1111 => {
            let secondary_group = secondary_group!(inst_word);
            log!(3, "The upper nibble is 0b1111, so let's inspect the secondary group: {:#05b}", secondary_group);
            match secondary_group {
                0b000 => {
                    let rd_index = rd_index!(inst_word);
                    log!(4, "The secondary group is 0b000, so let's inspect Rd: {:#05b}", rd_index);
                    if rd_index == 0b111 {
                        return_inst!(5, decoded_inst, DSI6{imm6: imm6!(inst_word)});
                    } else {
                        let bits_54 = (inst_word >> 4) & 0b11;
                        log!(5, "Rd is not 0b111, so inspect bits [5:4]: {:#04b}", bits_54);
                        match bits_54 {
                            0b00 => { return_inst!(6, decoded_inst, dec_MUL(inst_word)); },
                            0b10 => {
                                return_inst!(6, decoded_inst, DS_Access {
                                    w: ((inst_word >> 3) & 0b1) == 0b1,
                                    rs: dec_reg_from_index(rs_index!(inst_word)),
                                });
                            },
                            0b11 => { return_inst!(6, decoded_inst, FR_Access{w: ((inst_word >> 3) & 0b1) == 0b1, rs: dec_reg_from_index(rs_index!(inst_word))}); },
                            _ => { return_inst!(6, decoded_inst, Invalid); },
                        }
                    }
                },
                0b001 => {
                    let bit_9 = (inst_word >> 9) & 0b1;
                    log!(4, "The secondary group is 0b001, so let's inspect bit 9: {:#03b}", bit_9);
                    if bit_9 == 0b1 {
                        return_inst!(5, decoded_inst, Invalid);
                    } else {
                        //Lower 16 bits will be filled in decode_wg2
                        return_inst!(5, decoded_inst, CALL{a22: ((inst_word as u32) << 16) & 0b1111110000000000000000});
                    }
                },
                0b010 => {
                    let rd_index = rd_index!(inst_word);
                    log!(4, "The secondary group is 0b010, so let's inspect Rd: {:#05b}", rd_index);
                    if rd_index == 0b111 {
                        //Lower 16 bits will be filled in decode_wg2
                        return_inst!(5, decoded_inst, JMPF{a22: ((inst_word as u32) << 16) & 0b1111110000000000000000});
                    } else {
                        return_inst!(5, decoded_inst, dec_MULS(inst_word));
                    }
                },
                0b011 => {
                    let rd_index = rd_index!(inst_word);
                    log!(4, "The secondary group is 0b011, so let's inspect Rd: {:#05b}", rd_index);
                    if rd_index == 0b111 {
                        return_inst!(5, decoded_inst, JMPR);
                    } else {
                        return_inst!(5, decoded_inst, dec_MULS(inst_word));
                    }
                },
                0b100 => { return_inst!(4, decoded_inst, dec_MUL(inst_word)); },
                0b101 => {
                    let bit_5 = (inst_word >> 5) & 0b1;//Look at bit 5 first to split the opcode space in twoish
                    log!(4, "The secondary group is 0b101, so let's inspect bit 5: {:#03b}", bit_5);
                    if bit_5 == 0b1 {
                        let bits_210 = inst_word & 0b111;//Look at the lowest 3 bits to decide what it is
                        log!(5, "Bit 5 is set, so let's inspect the lowest 3 bits: {:#05b}", bits_210);
                        match inst_word & 0b111 {
                            0b000 => { return_inst!(6, decoded_inst, BREAK); },
                            0b001 => { return_inst!(6, decoded_inst, CALLR); },
                            0b010 => { return_inst!(6, decoded_inst, DIVS); },
                            0b011 => { return_inst!(6, decoded_inst, DIVQ); },
                            0b100 => { return_inst!(6, decoded_inst, EXP); },
                            0b101 => { return_inst!(6, decoded_inst, NOP); },
                            _ => { return_inst!(6, decoded_inst, Invalid); },
                        }
                    } else {
                        let bits_432 = (inst_word >> 2) & 0b111;//Look at bits 4:2 to split things further
                        log!(5, "Bit 5 is not set, so let's inspect the bits [4:2]: {:#05b}", bits_432);
                        match bits_432 {
                            0b000 => { return_inst!(6, decoded_inst, INT_SET{f: ((inst_word >> 1) & 0b1) == 0b1, i: (inst_word & 0b1) == 0b1}); },
                            0b001 => {
                                let bit_1 = (inst_word >> 1) & 0b1;
                                log!(6, "Bits [4:2] are 0b001, so let's inspect bit 1: {:#03b}", bit_1);
                                if bit_1 == 0b1 {
                                    return_inst!(7, decoded_inst, Fraction{fra: (inst_word & 0b1) == 0b1});
                                } else {
                                    return_inst!(7, decoded_inst, FIR_MOV{fir: (inst_word & 0b1) == 0b1});
                                }
                            },
                            0b010 => {
                                let bit_1 = (inst_word >> 1) & 0b1;
                                log!(6, "Bits [4:2] are 0b010, so let's inspect bit 1: {:#03b}", bit_1);
                                if bit_1 == 0b1 {
                                    return_inst!(7, decoded_inst, SECBANK{s: (inst_word & 0b1) == 0b1});
                                } else {
                                    return_inst!(7, decoded_inst, IRQ{i: (inst_word & 0b1) == 0b1});
                                }
                            },
                            0b011 => {
                                let bit_0 = inst_word & 0b1;
                                log!(6, "Bits [4:2] are 0b011, so let's inspect bit 0: {:#03b}", bit_0);
                                if bit_0 == 0b1 {
                                    return_inst!(7, decoded_inst, IRQ_Nest_Mode{n: ((inst_word >> 1) & 0b1) == 0b1});
                                } else {
                                    return_inst!(7, decoded_inst, FIQ{f: ((inst_word >> 1) & 0b1) == 0b1});
                                }
                            },
                            _ => { return_inst!(6, decoded_inst, Invalid); },
                        }
                    }
                },
                0b110 | 0b111 => { return_inst!(4, decoded_inst, dec_MULS(inst_word)); },
                _ => { debug_panic!(); },//This should never occur
            }
        },
        0b1110 => {
            log!(3, "The upper nibble is 0b1110, so let's check if this is a branch");

            let secondary_group = secondary_group!(inst_word);
            if (rd_index!(inst_word) == 0b111) && ((secondary_group == 0b000) || (secondary_group == 0b001)) {
                return_inst!(4, decoded_inst, dec_Branch(inst_word));
            } else {
                log!(4, "This isn't a branch, so let's inspect the secondary group: {:#05b}", secondary_group);
                match secondary_group {
                    0b000 => {
                        let bit_3 = (inst_word >> 3) & 0b1;
                        log!(5, "The secondary group is 0b000, so let's inspect bit 3: {:#03b}", bit_3);
                        if bit_3 == 0b1 {
                            return_inst!(6, decoded_inst, dec_MUL(inst_word));
                        } else {
                            return_inst!(6, decoded_inst, Register_BITOP_Rs {
                                rd: dec_reg_from_index(rd_index!(inst_word)),
                                op: dec_bit_op(inst_word),
                                rs: dec_reg_from_index(rs_index!(inst_word)),
                            });
                        }
                    },
                    0b001 => {
                        return_inst!(5, decoded_inst, Register_BITOP_offset {
                            rd: dec_reg_from_index(rd_index!(inst_word)),
                            op: dec_bit_op(inst_word),
                            offset: (inst_word & 0b1111) as u8,
                        });
                    },
                    0b010 | 0b011 => { return_inst!(5, decoded_inst, dec_MULS(inst_word)); },
                    0b100 | 0b101 => {
                        let bit_3 = (inst_word >> 3) & 0b1;
                        log!(5, "The secondary group is 0b000, so let's inspect bit 3: {:#03b}", bit_3);
                        if bit_3 == 0b1 {
                            return_inst!(6, decoded_inst, sixteen_bits_Shift {
                                rd: dec_reg_from_index(rd_index!(inst_word)),
                                op: dec_lsft_op(inst_word),
                                rs: dec_reg_from_index(rs_index!(inst_word)),
                            });
                        } else {
                            return_inst!(6, decoded_inst, Memory_BITOP_Rs {
                                rd: dec_reg_from_index(rd_index!(inst_word)),
                                d: ((inst_word >> 6) & 0b1) == 0b1,
                                op: dec_bit_op(inst_word),
                                rs: dec_reg_from_index(rs_index!(inst_word)),
                            });
                        }
                    },
                    0b110 | 0b111 => {
                        return_inst!(5, decoded_inst, Memory_BITOP_offset {
                            rd: dec_reg_from_index(rd_index!(inst_word)),
                            d: ((inst_word >> 6) & 0b1) == 0b1,
                            op: dec_bit_op(inst_word),
                            offset: (inst_word & 0b1111) as u8,
                        });
                    },
                    _ => { return_inst!(5, decoded_inst, Invalid); },
                }
            }
        },
        0b0101 | 0b0111 => {
            log!(3, "The upper nibble indicates this is likely a branch, verifying that it is valid...");
            let secondary_group = secondary_group!(inst_word);
            if (rd_index!(inst_word) == 0b111) && ((secondary_group == 0b000) || (secondary_group == 0b001)) {
                return_inst!(4, decoded_inst, dec_Branch(inst_word));
            } else {
                return_inst!(4, decoded_inst, Invalid);
            }
        },
        upper_nibble => {
            log!(3, "The upper nibble is {}, so let's check if this is a branch", upper_nibble);
            let secondary_group = secondary_group!(inst_word);
            if (rd_index!(inst_word) == 0b111) && ((secondary_group == 0b000) || (secondary_group == 0b001)) {
                return_inst!(4, decoded_inst, dec_Branch(inst_word));
            } else {
                log!(4, "This isn't a branch, so let's inspect the secondary group: {:#05b}", secondary_group);
                match secondary_group {
                    0b000 => { return_inst!(5, decoded_inst, Base_plus_Disp6{
                            op: dec_alu_op(inst_word),
                            rd: dec_reg_from_index(rd_index!(inst_word)),
                            imm6: imm6!(inst_word),
                        });
                    },
                    0b001 => {
                        return_inst!(5, decoded_inst, IMM6 {
                            op: dec_alu_op(inst_word),
                            rd: dec_reg_from_index(rd_index!(inst_word)),
                            imm6: imm6!(inst_word),
                        });
                    },
                    0b010 => {
                        log!(5, "The secondary group is 0b010, so let's look at the upper nibble to check if this is a PUSH Stack Operation");
                        if upper_nibble == 0b1101 {
                            return_inst!(6, decoded_inst, Stack_Operation {
                                op: DecodedStackOp::PUSH,
                                rd_index: rd_index!(inst_word) as u8,
                                size: ((inst_word >> 3) & 0b111) as u8,
                                rs: dec_reg_from_index(rs_index!(inst_word)),
                            });
                        } else {
                            log!(6, "Nope! Let's check if it is RETI or RETF");
                            if        inst_word == 0b1001101010011000 {
                                return_inst!(7, decoded_inst, RETI);
                            } else if inst_word == 0b1001101010010000 {
                                return_inst!(7, decoded_inst, RETF);
                            } else {
                                log!(7, "Nope! Let's check the upper nibble to see if it is POP; otherwise it's invalid");
                                if upper_nibble == 0b1001 {
                                    return_inst!(8, decoded_inst, Stack_Operation {
                                        op: DecodedStackOp::POP,
                                        rd_index: rd_index!(inst_word) as u8,
                                        size: ((inst_word >> 3) & 0b111) as u8,
                                        rs: dec_reg_from_index(rs_index!(inst_word)),
                                    });
                                } else {
                                    return_inst!(8, decoded_inst, Invalid);
                                }
                            }
                        }
                    },
                    0b011 => {
                        return_inst!(5, decoded_inst, DS_Indirect {
                            op: dec_alu_op(inst_word),
                            rd: dec_reg_from_index(rd_index!(inst_word)),
                            d: ((inst_word >> 5) & 0b1) == 0b1,
                            at: dec_at_op(inst_word),
                            rs: dec_reg_from_index(rs_index!(inst_word)),
                        });
                    },
                    0b100 => {
                        let bits_543 = (inst_word >> 3) & 0b111;
                        log!(5, "The secondary group is 0b100, so let's inspect bits [5:3]: {:#03b}", bits_543);
                        match bits_543 {
                            0b001 => {
                                return_inst!(6, decoded_inst, IMM16 {
                                    op: dec_alu_op(inst_word),
                                    rd: dec_reg_from_index(rd_index!(inst_word)),
                                    rs: dec_reg_from_index(rs_index!(inst_word)),
                                    imm16: 0,//imm16 will be filled in decode_wg2
                                });
                            },
                            0b010 | 0b011 => {
                                return_inst!(6, decoded_inst, Direct16 {
                                    op: dec_alu_op(inst_word),
                                    rd: dec_reg_from_index(rd_index!(inst_word)),
                                    w: ((inst_word >> 3) & 0b1) == 0b1,
                                    rs: dec_reg_from_index(rs_index!(inst_word)),
                                    a16: 0,//a16 will be filled in decode_wg2
                                });
                            },
                            _ => { return_inst!(6, decoded_inst, dec_Register(inst_word)); },
                        }
                    },
                    0b101 | 0b110 => { return_inst!(6, decoded_inst, dec_Register(inst_word)); },
                    0b111 => {
                        return_inst!(6, decoded_inst, Direct6 {
                            op: dec_alu_op(inst_word),
                            rd: dec_reg_from_index(rd_index!(inst_word)),
                            a6: imm6!(inst_word),
                        });
                    },
                    _ => { debug_panic!(); },//This should never occur
                }
            }
        },
    }
}

pub fn decode_wg2(decoded_inst: &mut DecodedInstruction, wg2: u16) {
    log!(1, "unSP: Decode instruction word group 2:");
    match decoded_inst {
        CALL{ref mut a22} => {
            log!(2, "Fill in the lower 16 bits of A22 for CALL");
            *a22 |= wg2 as u32;
            instruction_printing::log_addr!(3, "A22", *a22);
        },
        JMPF{ref mut a22} => {
            log!(2, "Fill in the lower 16 bits of A22 for JMPF");
            *a22 |= wg2 as u32;
            instruction_printing::log_addr!(3, "A22", *a22);
        },
        IMM16{ref mut imm16, ..} => {
            log!(2, "Get the 16-bit immediate for IMM16");
            *imm16 = wg2;
            instruction_printing::log_data!(3, "IMM16", wg2);
        },
        Direct16{ref mut a16, ..} => {
            log!(2, "Get the 16-bit address for Direct16");
            *a16 = wg2;
            instruction_printing::log_data!(3, "A16", wg2);
        },
        _ => { debug_panic!(); }//This instruction does not need to look at word group 2
    }
}

pub fn needs_decode_wg2(decoded_inst: &DecodedInstruction) -> bool {
    match decoded_inst {
        CALL{..} | JMPF{..} | IMM16{..} | Direct16{..} => { return true; }
        _ => { return false; }
    }
}

fn dec_alu_op(inst_word: u16) -> DecodedALUOp {
    use DecodedALUOp::*;
    match upper_nibble!(inst_word) {
        0b0000 => { return ADD; },
        0b0001 => { return ADC; },
        0b0010 => { return SUB; },
        0b0011 => { return SBC; },
        0b0100 => { return CMP; },
        0b0110 => { return NEG; },
        0b1000 => { return XOR; },
        0b1001 => { return LOAD; },
        0b1010 => { return OR; },
        0b1011 => { return AND; },
        0b1100 => { return TEST; },
        0b1101 => { return STORE; },
        _ => { return DecodedALUOp::Invalid; },
    }
}

fn dec_branch_op(inst_word: u16) -> DecodedBranchOp {
    use DecodedBranchOp::*;
    match upper_nibble!(inst_word) {
        0b0000 => { return JCC_JB_JNAE; },
        0b0001 => { return JCS_JNB_JAE; },
        0b0010 => { return JSC_JGE_JNL; },
        0b0011 => { return JSS_JNGE_JL; },
        0b0100 => { return JNE_JNZ; },
        0b0101 => { return JZ_JE; },
        0b0110 => { return JPL; },
        0b0111 => { return JMI; },
        0b1000 => { return JBE_JNA; },
        0b1001 => { return JNBE_JA; },
        0b1010 => { return JLE_JNG; },
        0b1011 => { return JNLE_JG; },
        0b1100 => { return JVC; },
        0b1101 => { return JVS; },
        0b1110 => { return JMP; },
        _ => { return DecodedBranchOp::Invalid; },
    }
}

fn dec_at_op(inst_word: u16) -> DecodedAtOp {
    use DecodedAtOp::*;
    match (inst_word >> 3) & 0b11 {
        0b00 => { return NOP; },
        0b01 => { return PostDecrement; },
        0b10 => { return PostIncrement; },
        0b11 => { return PreIncrement; },
        _ => { return debug_panic!(DecodedAtOp::Invalid); },//This should never occur
    }
}

fn dec_bit_op(inst_word: u16) -> DecodedBitOp {
    use DecodedBitOp::*;
    match (inst_word >> 4) & 0b11 {
        0b00 => { return TSTB; },
        0b01 => { return SETB; },
        0b10 => { return CLRB; },
        0b11 => { return INVB; },
        _ => { return debug_panic!(DecodedBitOp::Invalid); },//This should never occur
    }
}

fn dec_lsft_op(inst_word: u16) -> DecodedLSFTOp {
    use DecodedLSFTOp::*;
    match (inst_word >> 4) & 0b111 {
        0b000 => { return ASR; },
        0b001 => { return ASROR; },
        0b010 => { return LSL; },
        0b011 => { return LSLOR; },
        0b100 => { return LSR; },
        0b101 => { return LSROR; },
        0b110 => { return ROL; },
        0b111 => { return ROR; },
        _ => { return debug_panic!(DecodedLSFTOp::Invalid); },//This should never occur
    }
}

fn dec_sft_op(inst_word: u16) -> DecodedSFTOp {
    use DecodedSFTOp::*;
    match (inst_word >> 5) & 0b111 {
        0b000 => { return NOP; },
        0b001 => { return ASR; },
        0b010 => { return LSL; },
        0b011 => { return LSR; },
        0b100 => { return ROL; },
        0b101 => { return ROR; },
        _ => { return DecodedSFTOp::Invalid; },
    }
}

fn dec_reg_from_index(reg_index: u8) -> DecodedRegister {
    use DecodedRegister::*;
    match reg_index {
        0b000 => { return SP; },
        0b001 => { return R1_SR1; },
        0b010 => { return R2_SR2; },
        0b011 => { return R3_SR3; },
        0b100 => { return R4_SR4; },
        0b101 => { return BP; },
        0b110 => { return SR; },
        0b111 => { return PC; },
        _ => { return debug_panic!(DecodedRegister::Invalid); },//This should never occur
    }
}

#[allow(non_snake_case)]
fn dec_MUL(inst_word: u16) -> DecodedInstruction {
    return MUL {
        s_rs: ((inst_word >> 12) & 0b1) == 0b1,
        rd: dec_reg_from_index(rd_index!(inst_word)),
        s_rd: ((inst_word >> 8) & 0b1) == 0b1,
        rs: dec_reg_from_index(rs_index!(inst_word)),
    };
}

#[allow(non_snake_case)]
fn dec_MULS(inst_word: u16) -> DecodedInstruction {
    return MULS {
        s_rs: ((inst_word >> 12) & 0b1) == 0b1,
        rd: dec_reg_from_index(rd_index!(inst_word)),
        s_rd: ((inst_word >> 8) & 0b1) == 0b1,
        size: ((inst_word >> 3) & 0b1111) as u8,
        rs: dec_reg_from_index(rs_index!(inst_word)),
    };
}

#[allow(non_snake_case)]
fn dec_Branch(inst_word: u16) -> DecodedInstruction {
    return Branch {
        op: dec_branch_op(inst_word),
        d: ((inst_word >> 6) & 0b1) == 0b1,
        imm6: imm6!(inst_word),
    };
}

#[allow(non_snake_case)]
fn dec_Register(inst_word: u16) -> DecodedInstruction {
    return Register {
        op: dec_alu_op(inst_word),
        rd: dec_reg_from_index(rd_index!(inst_word)),
        sft: dec_sft_op(inst_word),
        sfc: ((inst_word >> 3) & 0b11) as u8,
        rs: dec_reg_from_index(rs_index!(inst_word)),
    }
}

/* Tests */

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn sanity() {
        let mut result: DecodedInstruction = DecodedInstruction::Invalid;
        decode_wg1(0x0000, &mut result);
        assert!(matches!(result, DecodedInstruction::Invalid));
        decode_wg1(0xFFFF, &mut result);
        assert!(matches!(result, DecodedInstruction::Invalid));
        decode_wg1(0b1001101010010000, &mut result);
        assert!(matches!(result, DecodedInstruction::RETF));
        decode_wg1(0b1111000101100101, &mut result);
        assert!(matches!(result, DecodedInstruction::NOP));
    }

    #[test]
    fn decode_all_possible_without_panic() {
        for wg1 in 0..=0xFFFF {
            let mut result_wg1: DecodedInstruction = DecodedInstruction::Invalid;
            decode_wg1(wg1, &mut result_wg1);
            if needs_decode_wg2(&result_wg1) {
                for wg2 in 0..=0xFFFF {
                    let mut result_wg2 = result_wg1.clone();
                    decode_wg2(&mut result_wg2, wg2);
                }
            }
        }
    }
}

//TESTING
pub fn add_two(a: i32) -> i32 {
    a + 2
}


#[cfg_attr(feature = "nightly-features", cfg(test))]
#[cfg(feature = "nightly-features")]
mod benches {
    use super::*;
    use test::Bencher;

    #[bench]
    fn bench_add_two(b: &mut Bencher) {
        b.iter(|| add_two(2));
    }
}
