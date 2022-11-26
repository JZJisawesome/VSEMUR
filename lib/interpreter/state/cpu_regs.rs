/* NAME//TODO
 * Copyright (C) 2022 John Jekel
 * See the LICENSE file at the root of the project for licensing info.
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

/* Tests */

#[cfg(test)]
mod tests {
    use super::*;
    use super::super::State;

    #[test]
    fn sanity() {
        use crate::interpreter::common::CPU;
        let mut state = init_new_random_state();
        state.reset_cpu();
        *state.reg_sp_mut() = 0xabcd;
        state.reg_r_mut()[0] = 0xdbca;
        state.reg_r_mut()[1] = 0x1234;
        state.reg_r_mut()[2] = 0x4321;
        state.reg_r_mut()[3] = 0x8421;
        *state.reg_bp_mut() = 0x5432;
        *state.reg_sr_mut() = 0x6969;
        *state.reg_pc_mut() = 0xDEAD;

        assert_eq!(state.cpu_regs.sp, 0xabcd);
        assert_eq!(state.cpu_regs.r[0], 0xdbca);
        assert_eq!(state.cpu_regs.r[1], 0x1234);
        assert_eq!(state.cpu_regs.r[2], 0x4321);
        assert_eq!(state.cpu_regs.r[3], 0x8421);
        assert_eq!(state.cpu_regs.bp, 0x5432);
        assert_eq!(state.cpu_regs.sr, 0x6969);
        assert_eq!(state.cpu_regs.pc, 0xDEAD);

        assert_eq!(*state.reg_sp(), 0xabcd);
        assert_eq!(state.reg_r()[0], 0xdbca);
        assert_eq!(state.reg_r()[1], 0x1234);
        assert_eq!(state.reg_r()[2], 0x4321);
        assert_eq!(state.reg_r()[3], 0x8421);
        assert_eq!(*state.reg_bp(), 0x5432);
        assert_eq!(*state.reg_sr(), 0x6969);
        assert_eq!(*state.reg_pc(), 0xDEAD);
    }

    #[test]
    fn proper_reset_values() {
        use crate::interpreter::common::CPU;
        let mut state = init_new_random_state();
        state.reset_cpu();
        assert_eq!(state.get_aq(), false);
        assert_eq!(state.get_bnk(), false);
        assert_eq!(state.get_fra(), false);
        assert_eq!(state.get_fir(), false);
        assert_eq!(state.get_sb(), 0b0000);
        assert_eq!(state.get_fiq(), false);
        assert_eq!(state.get_irq(), false);
        assert_eq!(state.get_ine(), false);
        assert_eq!(state.get_pri(), 0b1000);
        assert_eq!(state.get_cs(), 0x00);
        assert_eq!(state.get_pc(), state.read_addr((RESET_INT_VECTOR_ADDR & 0xFFFF) as u32));
    }

    #[test]
    fn proper_secbank_behaviour() {
        use crate::interpreter::common::CPU;
        let mut state = init_new_random_state();
        state.reset_cpu();
        state.cpu_regs.r[0] = 0x6543;
        state.cpu_regs.r[1] = 0x210F;
        state.cpu_regs.r[2] = 0xEDCB;
        state.cpu_regs.r[3] = 0xA987;
        state.cpu_regs.sec_r[0] = 0x0101;
        state.cpu_regs.sec_r[1] = 0xA55A;
        state.cpu_regs.sec_r[2] = 0x1337;
        state.cpu_regs.sec_r[3] = 0x0420;

        assert_eq!(state.reg_r()[0], 0x6543);
        assert_eq!(state.reg_r()[1], 0x210F);
        assert_eq!(state.reg_r()[2], 0xEDCB);
        assert_eq!(state.reg_r()[3], 0xA987);

        state.set_bnk(true);

        assert_eq!(state.reg_r()[0], 0x0101);
        assert_eq!(state.reg_r()[1], 0xA55A);
        assert_eq!(state.reg_r()[2], 0x1337);
        assert_eq!(state.reg_r()[3], 0x0420);
    }

    fn init_new_random_state() -> State {
        let mut state = State::new();
        state.load_bios_mem(&get_random_u16_slice(crate::interpreter::common::MAX_BIOS_SIZE_WORDS));
        state.load_rom_mem(&get_random_u16_slice(crate::interpreter::common::MAX_ROM_SIZE_WORDS));
        state.reset();
        return state;
    }

    fn get_random_u16_slice(size: usize) -> Box<[u16]> {
        let mut the_box = vec![0u16; size].into_boxed_slice();
        for i in 0..size {
            the_box[i] = i as u16;//TODO make actually random
        }
        return the_box;
    }
}
