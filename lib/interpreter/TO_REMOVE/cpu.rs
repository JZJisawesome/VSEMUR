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

use crate::debug_panic;

use crate::logging::log;
use super::common::MEM_SIZE_WORDS;
use crate::decode;

use super::common::InstructionMemory;
use super::common::ReadableMemory;
use super::common::WritableMemory;

/* Constants */

//Page 47 is useful :)
const BREAK_INT_VECTOR_ADDR: usize = 0xFFF5;
const FIQ_INT_VECTOR_ADDR: usize = 0xFFF6;
const RESET_INT_VECTOR_ADDR: usize = 0xFFF7;
const IRQ_INT_VECTOR_ADDR: [usize;8] = [0xFFF8, 0xFFF9, 0xFFFA, 0xFFFB, 0xFFFC, 0xFFFD, 0xFFFE, 0xFFFF];//0 thru 7

/* Macros */

//TODO

/* Static Variables */

//TODO

/* Types */

pub(super) struct CPUState {
    sp: u16,
    r: [u16;4],
    sec_r: [u16;4],
    bp: u16,
    sr: u16,
    pc: u16,
    fr: u16,

    cycle_count: u8,//Instructions may take multiple clock cycles; we fake this by waiting the proper amount of them after executing the whole thing on the first tick()

    //cache_valid: bool,//TODO actually read this value
    //decoded_instruction_cache: Box<[decode::DecodedInstruction]>,
}

/* Associated Functions and Methods */

impl CPUState {
    pub(super) fn new() -> CPUState {
        log!(1, "Initializing CPU");
        return CPUState {
            sp: 0,
            r: [0, 0, 0, 0],
            sec_r: [0, 0, 0, 0],
            bp: 0,
            sr: 0,
            pc: 0,
            fr: 0,

            cycle_count: 0,

            //cache_valid: false,
            //decoded_instruction_cache: vec![decode::DecodedInstruction::Invalid; 0].into_boxed_slice(),//TODO avoid allocating anything until we need it
        };
    }

    pub(super) fn reset(self: &mut Self, mem: &impl ReadableMemory) {
        log!(1, "Resetting CPU");

        log!(2, "Initialize FR to 0bx_0_0_0_0_0000_0_0_0_1000");
        self.fr = 0b0_0_0_0_0_0000_0_0_0_1000;

        log!(2, "Set initial CS page and PC");
        debug_assert!(RESET_INT_VECTOR_ADDR < MEM_SIZE_WORDS);
        log!(3, "Read reset vector at address {:#04X}_{:04X}", RESET_INT_VECTOR_ADDR >> 16, RESET_INT_VECTOR_ADDR & 0xFFFF);
        self.set_cs(0x00);
        self.pc = mem.read_addr(RESET_INT_VECTOR_ADDR as u32);
        log!(3, "Initial CS page, PC is {:#04X}_{:04X}", self.get_cs(), self.pc);

        //TODO do we need to initialize the cs or ds?
    }
    /*
    pub(super) fn cache(self: &mut Self, mem: &MemoryState) {
        log!(1, "Decoding and caching instructions...");

        self.decoded_instruction_cache = vec![decode::DecodedInstruction::Invalid; crate::interpreter::MEM_SIZE_WORDS].into_boxed_slice();

        for i in 0..crate::interpreter::MEM_SIZE_WORDS {

            //Fetch instruction from memory
            log!(1, "CPU: Fetch started from CS page, PC address: {:#04X}_{:04X}", i >> 16, i & 0xFFFF);
            let inst_word: u16 = mem.read_addr(i as u32);
            log!(2, "Instruction word group 1: {:#06X} | {:#018b}", inst_word, inst_word);

            //Decode it
            let mut decoded_inst = decode::DecodedInstruction::Invalid;
            decode::decode_wg1(inst_word, &mut decoded_inst);
            if decode::needs_decode_wg2(&decoded_inst) && (i != (crate::interpreter::MEM_SIZE_WORDS - 1)){
                log!(1, "CPU: Fetch started from CS page, PC address + 1");
                let wg2 = mem.read_addr((i + 1) as u32);
                log!(2, "Instruction word group 2: {:#06X} | {:#018b}", wg2, wg2);
                decode::decode_wg2(&mut decoded_inst, wg2);
            }

            //Add it to the cache
            self.decoded_instruction_cache[i] = decoded_inst;
        }

        self.cache_valid = true;
    }
    */

    /*
    pub(super) fn tick_old(self: &mut Self, mem: &mut MemoryState) {
        debug_assert!(mem.ready());

        //Wait for the proper number of cycles depending on the last instruction executed
        if self.cycle_count != 0 {
            log!(1, "CPU: Waiting {} more cycle(s) for the instruction to finish", self.cycle_count);
            log!(1, "CPU: CS page, PC is still {:#04X}_{:04X} | SP is still {:#04X}", self.get_cs(), self.pc, self.sp);
            self.cycle_count -= 1;
            return;
        }

        //Fetch instruction from memory
        debug_assert!(self.get_cs() < 0b111111);
        log!(1, "CPU: Fetch started from CS page, PC address: {:#04X}_{:04X}", self.get_cs(), self.pc);
        let inst_word: u16 = mem.read_page_addr(self.get_cs(), self.pc);
        log!(2, "Instruction word group 1: {:#06X} | {:#018b}", inst_word, inst_word);

        //Decode it
        let mut decoded_inst = decode::DecodedInstruction::Invalid;
        decode::decode_wg1(inst_word, &mut decoded_inst);
        if decode::needs_decode_wg2(&decoded_inst) {
            log!(1, "CPU: Fetch started from CS page, PC address + 1");
            let wg2 = get_wg2(self, mem);
            log!(2, "Instruction word group 2: {:#06X} | {:#018b}", wg2, wg2);
            decode::decode_wg2(&mut decoded_inst, wg2);
        }

        //Execute the decoded instruction
        execute::execute(self, mem, &decoded_inst);

        //TODO handle interrupts, etc

        log!(1, "CPU: CS page, PC is now {:#04X}_{:04X} | SP is now {:#04X}", self.get_cs(), self.pc, self.sp);
    }
    */

    pub(super) fn tick(self: &mut Self, mem: &mut (impl InstructionMemory + WritableMemory)) -> u8 {
        //TODO take advantange of the InstructionMemory's should_invalidate_icache function

        //Fetch instruction from memory
        debug_assert!(self.get_cs() < 0b111111);
        log!(1, "CPU: Fetch started from CS page, PC address: {:#04X}_{:04X}", self.get_cs(), self.pc);
        let inst_word: u16 = mem.fetch_page_addr(self.get_cs(), self.pc);
        log!(2, "Instruction word group 1: {:#06X} | {:#018b}", inst_word, inst_word);

        //Decode it
        let mut decoded_inst = decode::DecodedInstruction::Invalid;
        decode::decode_wg1(inst_word, &mut decoded_inst);
        if decode::needs_decode_wg2(&decoded_inst) {
            log!(1, "CPU: Fetch started from CS page, PC address + 1");
            let wg2 = get_wg2(self, mem);
            log!(2, "Instruction word group 2: {:#06X} | {:#018b}", wg2, wg2);
            decode::decode_wg2(&mut decoded_inst, wg2);
        }

        //Execute the decoded instruction
        execute::execute(self, mem, &decoded_inst);

        //TODO handle interrupts, etc

        log!(1, "CPU: CS page, PC is now {:#04X}_{:04X} | SP is now {:#04X}", self.get_cs(), self.pc, self.sp);
        log!(1, "CPU: Emulated {} clock cycles in one tick", self.cycle_count);
        return self.cycle_count;//Return how many CPU cycles we executed
    }

    /*
    pub(super) fn tick_cached(self: &mut Self, mem: &mut MemoryState) {
        debug_assert!(mem.ready());
        //TODO check that cache_valid is true (in debug builds only)

        //Wait for the proper number of cycles depending on the last instruction executed
        if self.cycle_count != 0 {
            log!(1, "CPU: Waiting {} more cycle(s) for the instruction to finish", self.cycle_count);
            log!(1, "CPU: CS page, PC is still {:#04X}_{:04X} | SP is still {:#04X}", self.get_cs(), self.pc, self.sp);
            self.cycle_count -= 1;
            return;
        }

        //Execute the decoded instruction from the cache
        let cache_index: u32 = ((self.get_cs() as u32) << 16) | (self.pc as u32);
        let decoded_inst = self.decoded_instruction_cache[cache_index as usize].clone();
        execute::execute(self, mem, &decoded_inst);
    }
    */

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

    fn get_cs(self: &Self) -> u8 {
        return (self.sr & 0b111111) as u8;
    }

    fn set_ds(self: &mut Self, value: u8) {
        debug_assert!(value < 0b111111);
        self.sr = (self.sr & 0b0000001111111111) | ((value as u16) << 10);
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

    fn set_cs(self: &mut Self, value: u8) {
        debug_assert!(value < 0b111111);
        self.sr = (self.sr & 0b1111111111000000) | (value as u16);
    }

    //FR getters and setters
    fn get_aq(self: &Self) -> bool {
        return ((self.fr >> 14) & 0b1) == 0b1;
    }

    fn get_bnk(self: &Self) -> bool {
        return ((self.fr >> 13) & 0b1) == 0b1;
    }

    fn get_fra(self: &Self) -> bool {
        return ((self.fr >> 12) & 0b1) == 0b1;
    }

    fn get_fir(self: &Self) -> bool {
        return ((self.fr >> 11) & 0b1) == 0b1;
    }

    fn get_sb(self: &Self) -> u8 {
        return ((self.fr >> 7) & 0b1111) as u8;
    }

    fn get_fiq(self: &Self) -> bool {
        return ((self.fr >> 6) & 0b1) == 0b1;
    }

    fn get_irq(self: &Self) -> bool {
        return ((self.fr >> 5) & 0b1) == 0b1;
    }

    fn get_ine(self: &Self) -> bool {
        return ((self.fr >> 4) & 0b1) == 0b1;
    }

    fn get_pri(self: &Self) -> u8 {
        return (self.fr & 0b1111) as u8;
    }

    fn set_aq(self: &mut Self, value: bool) {
        self.fr = (self.fr & 0b1011111111111111) | ((if value { 0b1 } else { 0b0 }) << 14);
    }

    fn set_bnk(self: &mut Self, value: bool) {
        self.fr = (self.fr & 0b1101111111111111) | ((if value { 0b1 } else { 0b0 }) << 13);
    }

    fn set_fra(self: &mut Self, value: bool) {
        self.fr = (self.fr & 0b1110111111111111) | ((if value { 0b1 } else { 0b0 }) << 12);
    }

    fn set_fir(self: &mut Self, value: bool) {
        self.fr = (self.fr & 0b1111011111111111) | ((if value { 0b1 } else { 0b0 }) << 11);
    }

    fn set_sb(self: &mut Self, value: u8) {
        debug_assert!(value < 0b1111);
        self.fr = (self.fr & 0b1111100001111111) | ((value as u16) << 7);
    }

    fn set_fiq(self: &mut Self, value: bool) {
        self.fr = (self.fr & 0b1111111110111111) | ((if value { 0b1 } else { 0b0 }) << 6);
    }

    fn set_irq(self: &mut Self, value: bool) {
        self.fr = (self.fr & 0b1111111111011111) | ((if value { 0b1 } else { 0b0 }) << 5);
    }

    fn set_ine(self: &mut Self, value: bool) {
        self.fr = (self.fr & 0b1111111111101111) | ((if value { 0b1 } else { 0b0 }) << 4);
    }

    fn set_pri(self: &mut Self, value: u8) {
        debug_assert!(value < 0b1111);
        self.fr = (self.fr & 0b1111111111110000) | (value as u16);
    }

    //MR getter and setter
    fn get_mr(self: &Self) -> u32 {
        return ((self.get_reg(decode::DecodedRegister::R4_SR4) as u32) << 16) | (self.get_reg(decode::DecodedRegister::R3_SR3) as u32);
    }

    fn set_mr(self: &mut Self, value: u32) {
        self.set_reg(decode::DecodedRegister::R4_SR4, ((value >> 16) & 0xFFFF) as u16);
        self.set_reg(decode::DecodedRegister::R3_SR3, (value & 0xFFFF) as u16);
    }

    //Regular registers
    fn get_reg(self: &Self, reg: decode::DecodedRegister) -> u16 {
        use decode::DecodedRegister::*;
        match reg {
            SP => { return self.sp; },
            R1_SR1 => { return if self.get_bnk() { self.sec_r[0] } else {self.r[0]}; },
            R2_SR2 => { return if self.get_bnk() { self.sec_r[1] } else {self.r[1]}; },
            R3_SR3 => { return if self.get_bnk() { self.sec_r[2] } else {self.r[2]}; },
            R4_SR4 => { return if self.get_bnk() { self.sec_r[3] } else {self.r[3]}; },
            BP => { return self.bp; },
            SR => { return self.sr; },
            PC => { return self.pc; },

            Invalid => { return debug_panic!(0); }//We shouldn't be passed this
        }
    }

    fn set_reg(self: &mut Self, reg: decode::DecodedRegister, value: u16) {
        use decode::DecodedRegister::*;
        match reg {
            SP => { self.sp = value; },
            R1_SR1 => { if self.get_bnk() { self.sec_r[0] = value; } else { self.r[0] = value; } },
            R2_SR2 => { if self.get_bnk() { self.sec_r[1] = value; } else { self.r[1] = value; } },
            R3_SR3 => { if self.get_bnk() { self.sec_r[2] = value; } else { self.r[2] = value; } },
            R4_SR4 => { if self.get_bnk() { self.sec_r[3] = value; } else { self.r[3] = value; } },
            BP => { self.bp = value; },
            SR => { self.sr = value; },
            PC => { self.pc = value; },

            Invalid => { debug_panic!(); }//We shouldn't be passed this
        }
    }

    fn get_reg_by_index(self: &Self, reg: u8) -> u16 {
        debug_assert!(reg < 8);
        match reg {
            0b000 => { return self.sp; },
            0b001 => { return if self.get_bnk() { self.sec_r[0] } else { self.r[0] }; },
            0b010 => { return if self.get_bnk() { self.sec_r[1] } else { self.r[1] }; },
            0b011 => { return if self.get_bnk() { self.sec_r[2] } else { self.r[2] }; },
            0b100 => { return if self.get_bnk() { self.sec_r[3] } else { self.r[3] }; },
            0b101 => { return self.bp; },
            0b110 => { return self.sr; },
            0b111 => { return self.pc; },
            _ => { return debug_panic!(0); },//This should never occur
        }
    }

    fn set_reg_by_index(self: &mut Self, reg: u8, value: u16) {
        match reg {
            0b000 => { self.sp = value; },
            0b001 => { if self.get_bnk() { self.sec_r[0] = value; } else { self.r[0] = value; } },
            0b010 => { if self.get_bnk() { self.sec_r[1] = value; } else { self.r[1] = value; } },
            0b011 => { if self.get_bnk() { self.sec_r[2] = value; } else { self.r[2] = value; } },
            0b100 => { if self.get_bnk() { self.sec_r[3] = value; } else { self.r[3] = value; } },
            0b101 => { self.bp = value; },
            0b110 => { self.sr = value; },
            0b111 => { self.pc = value; },

            _ => { debug_panic!(); }//We shouldn't be passed this
        }
    }

    //Misc
    fn set_cycle_count(self: &mut Self, value: u8) {
        self.cycle_count = value;
    }

    //TESTING
    fn reg_sp(self: &mut Self) -> &mut u16 {
        return &mut self.sp;
    }
}

/* Functions */

use super::common::inc_page_addr_by;
use super::common::dec_page_addr_by;

fn get_wg2(cpu: &CPUState, mem: &impl InstructionMemory) -> u16 {
    let address_after_pc_tuple = inc_page_addr_by(cpu.get_cs(), cpu.pc, 1);
    return mem.fetch_page_addr(address_after_pc_tuple.0, address_after_pc_tuple.1);
}
