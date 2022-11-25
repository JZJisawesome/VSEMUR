/* NAME//TODO
 * By: John Jekel
 *
 * TODO description
 *
*/

/* Imports */

use super::State;

use crate::logging::log;

use crate::interpreter::common::CPU;

use crate::interpreter::common::MEM_SIZE_WORDS;

use crate::interpreter::common::ReadableMemory;

/* Constants */

//Page 47 is useful :)
const BREAK_INT_VECTOR_ADDR: usize = 0xFFF5;
const FIQ_INT_VECTOR_ADDR: usize = 0xFFF6;
const RESET_INT_VECTOR_ADDR: usize = 0xFFF7;
const IRQ_INT_VECTOR_ADDR: [usize;8] = [0xFFF8, 0xFFF9, 0xFFFA, 0xFFFB, 0xFFFC, 0xFFFD, 0xFFFE, 0xFFFF];//0 thru 7

/* Macros */

//TODO (also pub(crate) use the_macro statements here too)

/* Static Variables */

//TODO

/* Types */

#[derive(Default)]
pub(super) struct CPURegs {
    pub(super) sp: u16,
    pub(super) r: [u16;4],
    pub(super) sec_r: [u16;4],
    pub(super) bp: u16,
    pub(super) sr: u16,
    pub(super) pc: u16,
    pub(super) fr: u16,
}

/* Associated Functions and Methods */

impl State {
    pub(super) fn reset_cpu(self: &mut Self) {
        log!(2, "Resetting CPU");

        log!(3, "Initialize FR to 0bx_0_0_0_0_0000_0_0_0_1000");
        self.cpu_regs.fr = 0b0_0_0_0_0_0000_0_0_0_1000;

        log!(3, "Set initial CS page and PC");
        debug_assert!(RESET_INT_VECTOR_ADDR < MEM_SIZE_WORDS);
        log!(4, "Read reset vector at address {:#04X}_{:04X}", RESET_INT_VECTOR_ADDR >> 16, RESET_INT_VECTOR_ADDR & 0xFFFF);
        self.set_cs(0x00);
        self.cpu_regs.pc = self.read_addr(RESET_INT_VECTOR_ADDR as u32);
        log!(3, "Initial CS page, PC is {:#04X}_{:04X}", self.get_cs(), self.cpu_regs.pc);
    }
}

impl CPU for State {
    fn reg_sp(self: &Self) -> &u16 {
        return &self.cpu_regs.sp;
    }
    fn reg_r(self: &Self) -> &[u16;4] {
        return if self.get_bnk() { &self.cpu_regs.sec_r } else { &self.cpu_regs.r };
    }
    fn reg_bp(self: &Self) -> &u16 {
        return &self.cpu_regs.bp;
    }
    fn reg_sr(self: &Self) -> &u16 {
        return &self.cpu_regs.sr;
    }
    fn reg_pc(self: &Self) -> &u16 {
        return &self.cpu_regs.pc;
    }
    fn reg_fr(self: &Self) -> &u16 {
        return &self.cpu_regs.fr;
    }
    fn reg_sp_mut(self: &mut Self) -> &mut u16 {
        return &mut self.cpu_regs.sp;
    }
    fn reg_r_mut(self: &mut Self) -> &mut [u16;4] {
        return if self.get_bnk() { &mut self.cpu_regs.sec_r } else { &mut self.cpu_regs.r };
    }
    fn reg_bp_mut(self: &mut Self) -> &mut u16 {
        return &mut self.cpu_regs.bp;
    }
    fn reg_sr_mut(self: &mut Self) -> &mut u16 {
        return &mut self.cpu_regs.sr;
    }
    fn reg_pc_mut(self: &mut Self) -> &mut u16 {
        return &mut self.cpu_regs.pc;
    }
    fn reg_fr_mut(self: &mut Self) -> &mut u16 {
        return &mut self.cpu_regs.fr;
    }

    fn soft_interrupt_request(self: &mut Self) {
        todo!();
    }
}

/* Functions */

//TODO
