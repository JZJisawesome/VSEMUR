/* control.rs
 * By: John Jekel
 *
 * Emulates the control-transfer instructions of the CPU
 *
*/

//TODO remove this once everything is implemented
#![allow(unused_variables)]
#![allow(dead_code)]
#[allow(unused_imports)]

/* Imports */

use crate::debug_panic;
use crate::logging::log;
use crate::logging::log_noln;
use crate::logging::log_finln;
use crate::interpreter::common::Memory;
use super::CPUState;
use crate::decode::*;//TODO only import what is needed from here

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

pub(super) fn execute(cpu: &mut CPUState, mem: &mut impl Memory, inst: &DecodedInstruction) {
    use DecodedInstruction::*;
    match inst {
        CALL{a22} => {
            //Push the current PC, followed by the current SR, to the stack
            log!(3, "Push the current PC + 2, followed by the current SR, to the stack");
            //HACK We assume the SP will always point to page 0 (where memory is on the vsmile), so we never update the ds register here for speed
            cpu.inc_pc_by(2);//The PC isn't actually kept like this, it is just an easy way to increment it (also affecting the CS field properly too)
            mem.write_page_addr(0x00, cpu.sp, cpu.pc);
            cpu.sp -= 1;
            mem.write_page_addr(0x00, cpu.sp, cpu.sr);
            cpu.sp -= 1;

            //Update the CS (which is contained within SR) and the PC
            log!(3, "The CS becomes the high 6 bits of A22, and the PC becomes the low 16 bits");
            cpu.set_cs(((a22 >> 16) & 0b111111) as u8);
            cpu.pc = (a22 & 0xFFFF) as u16;
        },
        RETF => {
            //Pop the current PC, followed by the current SR, from the stack
            log!(3, "Pop the SR, followed by the PC, from the stack");
            cpu.sp += 1;
            cpu.sr = mem.read_page_addr(0x00, cpu.sp);//TODO do we take the whole SR, or just CS and discard the rest?
            cpu.sp += 1;
            cpu.pc = mem.read_page_addr(0x00, cpu.sp);
        },
        Branch{op, d, imm6} => {
            log!(3, "Let's look at the CPU's flags:");
            log!(4, "N: {}", cpu.get_n());
            log!(4, "Z: {}", cpu.get_z());
            log!(4, "S: {}", cpu.get_s());
            log!(4, "C: {}", cpu.get_c());
            if branch_taken(cpu, *op) {
                log!(3, "The branch is taken");
                cpu.set_cycle_count(4);

                let original_cs = cpu.get_cs();
                let original_pc = cpu.pc;

                let new_cs_pc_tuple: (u8, u16);
                if *d {
                    log!(4, "We're branching backwards!");
                    new_cs_pc_tuple = super::super::dec_page_addr_by(original_cs, original_pc, *imm6 as u32);
                } else {
                    log!(4, "We're branching forwards!");
                    new_cs_pc_tuple = super::super::inc_page_addr_by(original_cs, original_pc, *imm6 as u32);
                }

                cpu.set_cs(new_cs_pc_tuple.0);
                cpu.pc = new_cs_pc_tuple.1;
            } else {
                log!(3, "The branch is not taken");
                cpu.set_cycle_count(2);
            }

            //We always increment the PC for branches no matter what
            cpu.inc_pc();
        }
        //TODO others
        _ => { debug_panic!(); },//We should not have recieved this type of instruction
    }
}

fn branch_taken(cpu: &CPUState, op: DecodedBranchOp) -> bool {
    use super::super::decode::DecodedBranchOp::*;
    match op {
        JCC_JB_JNAE => { return cpu.get_c() == false; },
        JCS_JNB_JAE => { return cpu.get_c() == true; },
        JSC_JGE_JNL => { return cpu.get_s() == false; },
        JSS_JNGE_JL => { return cpu.get_s() == true; },
        JNE_JNZ => { return cpu.get_z() == false; },
        JZ_JE => { return cpu.get_s() == true; },
        JPL => { return cpu.get_n() == false; },
        JMI => { return cpu.get_n() == true; },
        JBE_JNA => { return !((cpu.get_z() == false) && (cpu.get_c() == true)); },
        JNBE_JA => { return (cpu.get_z() == false) && (cpu.get_c() == true); },
        JLE_JNG => { return !((cpu.get_z() == false) && (cpu.get_s() == false)); },
        JNLE_JG => { return (cpu.get_z() == false) && (cpu.get_s() == false); },
        JVC => { return cpu.get_n() == cpu.get_s(); },
        JVS => { return cpu.get_n() != cpu.get_s(); },
        JMP => { return true; },

        Invalid => { return debug_panic!(false); }
    }
}
