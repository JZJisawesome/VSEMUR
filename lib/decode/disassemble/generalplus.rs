/* NAME//TODO
 * By: John Jekel
 *
 * TODO description
 *
*/

//TODO remove this once everything is implemented
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(dead_code)]

/* Imports */

use crate::debug_panic;
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
