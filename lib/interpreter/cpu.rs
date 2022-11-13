/* cpu.rs
 * By: John Jekel
 *
 * Emulation for the CPU of the VSmile
 *
*/

//TODO remove this once everything is implemented
#![allow(unused_variables)]
#![allow(dead_code)]
#[allow(unused_imports)]

/* Imports */

mod execute;

use crate::logging::log;
use super::memory::MemoryState;
use super::MEM_SIZE_WORDS;

/* Constants */

const INT_VECTOR_BASE_ADDR: usize = 0xFFF5;//Page 47 is useful :)
const RESET_VECTOR_ADDR: usize = 0xFFF7;//Page 47 is useful :)

/* Macros */

//TODO (also pub(crate) use the_macro statements here too)

/* Static Variables */

//TODO

/* Types */

pub(super) struct CPUState {
    sp: u16,
    r: [u16;4],
    bp: u16,
    sr: u16,
    pc: u16,
    irq_enabled: bool,
    fiq_enabled: bool,

    cycle_count: u8,//Instructions may take multiple clock cycles; we fake this by waiting the proper amount of them after executing the whole thing on the first tick()
}

struct Inst {
    wg: [u16; 2],
}

/* Associated Functions and Methods */

impl CPUState {
    pub(super) fn new() -> CPUState {
        log!(0, 1, "Initializing CPU");
        return CPUState {
            sp: 0,
            r: [0, 0, 0, 0],
            bp: 0,
            sr: 0,
            pc: 0,
            irq_enabled: false,
            fiq_enabled: false,

            cycle_count: 0,
        };
    }

    pub(super) fn reset(self: &mut Self, mem: &MemoryState) {
        log!(0, 1, "Resetting CPU");

        log!(0, 2, "Zero out SP, R1, R2, R3, R4, BP, and SR");
        self.sp = 0;
        self.r = [0, 0, 0, 0];
        self.bp = 0;
        self.sr = 0;

        log!(0, 2, "Disable interrupts");
        self.irq_enabled = false;
        self.fiq_enabled = false;

        log!(0, 2, "Set initial CS page and PC");
        debug_assert!(RESET_VECTOR_ADDR < MEM_SIZE_WORDS);
        log!(0, 3, "Read reset vector at address {:#04X}_{:04X}", RESET_VECTOR_ADDR >> 16, RESET_VECTOR_ADDR & 0xFFFF);
        self.pc = mem.read_addr(RESET_VECTOR_ADDR as u32);
        log!(0, 3, "Initial CS page, PC is {:#04X}_{:04X}", self.get_cs(), self.pc);

        //TODO do we need to initialize the cs or ds?
    }

    pub(super) fn tick(self: &mut Self, t: u128, mem: &mut MemoryState) {
        debug_assert!(mem.ready());

        //Wait for the proper number of cycles depending on the last instruction executed
        if self.cycle_count != 0 {
            log!(t, 1, "CPU: Waiting {} more cycle(s) for the instruction to finish.", self.cycle_count);
            log!(t, 1, "CPU: CS page, PC is still {:#04X}_{:04X} | SP is still {:#04X}", self.get_cs(), self.pc, self.sp);
            self.cycle_count -= 1;
            return;
        }

        //Fetch instruction from memory
        debug_assert!(self.get_cs() < 0b111111);
        log!(t, 1, "CPU: Fetch started from CS page, PC address: {:#04X}_{:04X}", self.get_cs(), self.pc);
        let inst_word: u16 = mem.read_page_addr(self.get_cs(), self.pc);
        log!(t, 2, "Instruction word group 1:     {:#06X} | {:#018b}", inst_word, inst_word);

        //Execute it
        execute::execute(t, self, mem, inst_word);

        //TODO handle interrupts, etc

        log!(t, 1, "CPU: CS page, PC is now {:#04X}_{:04X} | SP is now {:#04X}", self.get_cs(), self.pc, self.sp);
    }

    //Make PC access easier (also handles the CS register if it needs to be incremented too)
    fn inc_pc_by(self: &mut Self, increment_amount: u32) {
        let result: (u8, u16) = inc_page_addr_by(self.get_cs(), self.pc, increment_amount);
        self.set_cs(result.0);
        self.pc = result.1;
    }

    fn inc_pc(self: &mut Self) {
        self.inc_pc_by(1);
    }

    //SR getters and setters for sub-fields
    fn get_ds(self: &Self) -> u8 {
        return ((self.sr >> 10) & 0b111111) as u8;
    }

    fn get_cs(self: &Self) -> u8 {
        return (self.sr & 0b111111) as u8;
    }

    fn get_n(self: &Self) -> bool {
        return ((self.sr >> 9) & 0b1) == 0b1;
    }

    fn get_z(self: &Self) -> bool {
        return ((self.sr >> 8) & 0b1) == 0b1;
    }

    fn get_s(self: &Self) -> bool {
        return ((self.sr >> 7) & 0b1) == 0b1;
    }

    fn get_c(self: &Self) -> bool {
        return ((self.sr >> 6) & 0b1) == 0b1;
    }

    fn set_ds(self: &mut Self, value: u8) {
        debug_assert!(value < 0b111111);
        self.sr = (self.sr & 0b0000001111111111) | ((value as u16) << 10);
    }

    fn set_cs(self: &mut Self, value: u8) {
        debug_assert!(value < 0b111111);
        self.sr = (self.sr & 0b1111111111000000) | (value as u16);
    }

    fn set_n(self: &mut Self, value: bool) {
        self.sr = (self.sr & 0b1111110111111111) | ((if value { 0b1 } else { 0b0 }) << 9);
    }

    fn set_z(self: &mut Self, value: bool) {
        self.sr = (self.sr & 0b1111111011111111) | ((if value { 0b1 } else { 0b0 }) << 8);
    }

    fn set_s(self: &mut Self, value: bool) {
        self.sr = (self.sr & 0b1111111101111111) | ((if value { 0b1 } else { 0b0 }) << 7);
    }

    fn set_c(self: &mut Self, value: bool) {
        self.sr = (self.sr & 0b1111111110111111) | ((if value { 0b1 } else { 0b0 }) << 6);
    }

    fn set_cycle_count(self: &mut Self, value: u8) {
        debug_assert!(value >= 1);
        self.cycle_count = value - 1;//Since the current cycle counts as the first one we must wait
    }
}

/* Functions */

fn inc_page_addr_by(page: u8, addr: u16, increment_amount: u32) -> (u8, u16) {
    //TODO error checking
    if ((addr as u32) + increment_amount) <= 0xFFFF {
        return (page, addr + (increment_amount as u16));//Safe since at most increment_amount could be 0xFFFF
    } else {
        unimplemented!();//TODO
    }
}
