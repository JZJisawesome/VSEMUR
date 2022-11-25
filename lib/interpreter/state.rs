/* state.rs
 * By: John Jekel
 *
 * TODO description
 *
*/

/* Imports */

use crate::debug_panic;

use crate::logging::log;

use super::common::Interrupt;
use super::common::CPU;
use super::common::InstructionMemory;
use super::common::ReadableMemory;
use super::common::WritableMemory;
use super::common::InterruptReadable;
use super::common::InterruptClearable;

/* Constants */

//TODO

/* Macros */

//TODO (also pub(crate) use the_macro statements here too)

/* Static Variables */

//TODO

/* Types */

#[derive(Default)]
pub(super) struct CPURegs {
    sp: u16,
    r: [u16;4],
    sec_r: [u16;4],
    bp: u16,
    sr: u16,
    pc: u16,
    fr: u16,
}

#[derive(Default)]
struct State {
    cpu_regs: CPURegs,
}

/* Associated Functions and Methods */

impl State {
    pub(super) fn new() -> State {
        log!(1, "Initializing VSmile state");
        return State::default();
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

impl InstructionMemory for State {
    fn should_invalidate_icache(self: &Self) -> bool {
        todo!();
    }

    fn fetch_addr(self: &Self, addr: u32) -> u16 {//For instruction fetching only (faster)
        todo!();
    }
}

impl ReadableMemory for State {
    fn read_addr(self: &Self, addr: u32) -> u16 {
        todo!();
    }
}

impl WritableMemory for State {
    fn write_addr(self: &mut Self, addr: u32, data: u16) {
        todo!();
    }
}

impl InterruptReadable for State {
    fn get_interrupt(self: &Self) -> Option<Interrupt> {
        todo!();
    }
}

impl InterruptClearable for State {
    fn clear_current_interrupt(self: &mut Self) {
        todo!();
    }
}

/* Functions */

//TODO
