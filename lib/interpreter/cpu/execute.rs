/* execute.rs
 * By: John Jekel
 *
 * Module for executing unSP instructions and performing other operations
 *
*/

//TODO remove this once everything is implemented
#![allow(unused_variables)]
#![allow(dead_code)]

/* Imports */

use crate::logging::log;
use crate::logging::log_noln;
use crate::logging::log_finln;
use crate::interpreter::memory::MemoryState;
use super::CPUState;
use super::decode::*;//TODO only import what is needed
use super::decode::DecodedInstruction::*;

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

pub(super) fn execute(cpu: &mut CPUState, mem: &mut MemoryState, inst: &DecodedInstruction) {
    log!(1, "CPU: Execute instruction word group 1");

    match inst {
        DSI6{..} => {
            unimplemented!();//TODO
        },
        CALL{..} => {
            unimplemented!();//TODO
        },
        JMPF{..} => {
            unimplemented!();//TODO
        },
        JMPR{..} => {
            unimplemented!();//TODO
        },
        FIR_MOV{..}=> {
            unimplemented!();//TODO
        },
        Fraction{..} => {
            unimplemented!();//TODO
        },
        INT_SET{..} => {
            unimplemented!();//TODO
        },
        IRQ{..} => {
            unimplemented!();//TODO
        },
        SECBANK{..} => {
            unimplemented!();//TODO
        },
        FIQ{..} => {
            unimplemented!();//TODO
        },
        IRQ_Nest_Mode{..} => {
            unimplemented!();//TODO
        },
        BREAK{..} => {
            unimplemented!();//TODO
        },
        CALLR{..} => {
            unimplemented!();//TODO
        },
        DIVS{..} => {
            unimplemented!();//TODO
        },
        DIVQ{..} => {
            unimplemented!();//TODO
        },
        EXP{..} => {
            unimplemented!();//TODO
        },
        NOP{..} => {
            unimplemented!();//TODO
        },
        DS_Access{..} => {
            unimplemented!();//TODO
        },
        FR_Access{..} => {
            unimplemented!();//TODO
        },
        MUL{..} => {
            unimplemented!();//TODO
        },
        MULS{..} => {
            unimplemented!();//TODO
        },
        Register_BITOP_Rs{..} => {
            unimplemented!();//TODO
        },
        Register_BITOP_offset{..} => {
            unimplemented!();//TODO
        },
        Memory_BITOP_offset{..} => {
            unimplemented!();//TODO
        },
        Memory_BITOP_Rs{..} => {
            unimplemented!();//TODO
        },
        sixteen_bits_Shift{..} => {
            unimplemented!();//TODO
        },
        RETI{..} => {
            unimplemented!();//TODO
        },
        RETF{..} => {
            unimplemented!();//TODO
        },
        Base_plus_Disp6{..} => {
            unimplemented!();//TODO
        },
        IMM6{..} => {
            unimplemented!();//TODO
        },
        Branch{..} => {
            unimplemented!();//TODO
        },
        Stack_Operation{..} => {
            unimplemented!();//TODO
        },
        DS_Indirect{..} => {
            unimplemented!();//TODO
        },
        IMM16{..} => {
            unimplemented!();//TODO
        },
        Direct16{..} => {
            unimplemented!();//TODO
        },
        Direct6{..} => {
            unimplemented!();//TODO
        },
        Register{..} => {
            unimplemented!();//TODO
        },

        InvalidInstructionType => { panic!(); }//TODO proper error handling

    }
}
