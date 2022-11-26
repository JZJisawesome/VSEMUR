/* control.rs
 * Copyright (C) 2022 John Jekel
 * See the LICENSE file at the root of the project for licensing info.
 *
 * Emulates the control-transfer instructions of the CPU
 *
*/

//TODO remove this once everything is implemented
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(dead_code)]

/* Imports */

use crate::debug_panic;

use crate::logging::log;
use crate::logging::log_noln;
use crate::logging::log_finln;

use crate::interpreter::common::CPU;
use crate::interpreter::common::InstructionMemory;
use crate::interpreter::common::ReadableMemory;
use crate::interpreter::common::WritableMemory;
use crate::interpreter::common::InterruptClearable;

use crate::decode::*;//TODO only import what is needed from here

use crate::interpreter::common::inc_page_addr_by;
use crate::interpreter::common::dec_page_addr_by;

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

pub(super) fn execute(state: &mut (impl CPU + ReadableMemory + WritableMemory + InterruptClearable), inst: &DecodedInstruction) -> u8 {
    use DecodedInstruction::*;
    match inst {
        CALL{a22} => {
            //Push the current PC, followed by the current SR, to the stack
            log!(3, "Push the current PC + 2, followed by the current SR, to the stack");
            //HACK We assume the SP will always point to page 0 (where memory is on the vsmile), so we never update the ds register here for speed
            state.inc_pc_by(2);//The PC isn't actually kept like this, it is just an easy way to increment it (also affecting the CS field properly too)
            state.write_page_addr(0x00, state.get_sp(), state.get_pc());
            *state.reg_sp_mut() -= 1;
            state.write_page_addr(0x00, state.get_sp(), state.get_sr());
            *state.reg_sp_mut() -= 1;

            //Update the CS (which is contained within SR) and the PC
            log!(3, "The CS becomes the high 6 bits of A22, and the PC becomes the low 16 bits");
            state.set_cs(((a22 >> 16) & 0b111111) as u8);
            state.set_pc((a22 & 0xFFFF) as u16);

            return 9;
        },
        RETF => {
            //Pop the current PC, followed by the current SR, from the stack
            log!(3, "Pop the SR, followed by the PC, from the stack");
            *state.reg_sp_mut() += 1;
            state.set_sr(state.read_page_addr(0x00, state.get_sp()));//TODO do we take the whole SR, or just CS and discard the rest?
            *state.reg_sp_mut() += 1;
            state.set_pc(state.read_page_addr(0x00, state.get_sp()));

            return 8;
        },
        Branch{op, d, imm6} => {
            log!(3, "Let's look at the CPU's flags:");
            log!(4, "N: {}", state.get_n());
            log!(4, "Z: {}", state.get_z());
            log!(4, "S: {}", state.get_s());
            log!(4, "C: {}", state.get_c());
            if branch_taken(state, *op) {
                log!(3, "The branch is taken");

                let original_cs = state.get_cs();
                let original_pc = state.get_pc();

                let new_cs_pc_tuple: (u8, u16);
                if *d {
                    log!(4, "We're branching backwards!");
                    new_cs_pc_tuple = dec_page_addr_by(original_cs, original_pc, *imm6 as u32);
                } else {
                    log!(4, "We're branching forwards!");
                    new_cs_pc_tuple = inc_page_addr_by(original_cs, original_pc, *imm6 as u32);
                }

                state.set_cs(new_cs_pc_tuple.0);
                state.set_pc(new_cs_pc_tuple.1);

                //We always increment the PC for branches no matter what
                state.inc_pc();
                return 4;
            } else {
                log!(3, "The branch is not taken");

                //We always increment the PC for branches no matter what
                state.inc_pc();
                return 2;
            }
        }
        //TODO others
        _ => { return debug_panic!(0); },//We should not have recieved this type of instruction
    }
}

fn branch_taken(state: &impl CPU, op: DecodedBranchOp) -> bool {
    use DecodedBranchOp::*;
    match op {
        JCC_JB_JNAE => { return state.get_c() == false; },
        JCS_JNB_JAE => { return state.get_c() == true; },
        JSC_JGE_JNL => { return state.get_s() == false; },
        JSS_JNGE_JL => { return state.get_s() == true; },
        JNE_JNZ => { return state.get_z() == false; },
        JZ_JE => { return state.get_s() == true; },
        JPL => { return state.get_n() == false; },
        JMI => { return state.get_n() == true; },
        JBE_JNA => { return !((state.get_z() == false) && (state.get_c() == true)); },
        JNBE_JA => { return (state.get_z() == false) && (state.get_c() == true); },
        JLE_JNG => { return !((state.get_z() == false) && (state.get_s() == false)); },
        JNLE_JG => { return (state.get_z() == false) && (state.get_s() == false); },
        JVC => { return state.get_n() == state.get_s(); },
        JVS => { return state.get_n() != state.get_s(); },
        JMP => { return true; },

        Invalid => { return debug_panic!(false); }
    }
}
