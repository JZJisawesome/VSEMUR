/* NAME//TODO
 * By: John Jekel
 *
 * TODO description
 *
*/

/* Imports */

use super::State;

use crate::interpreter::common::CPU;

/* Constants */

//TODO

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

/* Associated Functions and Methods */

//TODO

/* Functions */

//TODO
