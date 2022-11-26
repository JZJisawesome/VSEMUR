/* NAME//TODO
 * By: John Jekel
 *
 * TODO description
 *
*/

/* Imports */

use crate::debug_panic;

use super::State;

use crate::logging::log;
use crate::logging::log_ansi;

use crate::interpreter::common::CPU;
use crate::interpreter::common::InstructionMemory;
use crate::interpreter::common::ReadableMemory;
use crate::interpreter::common::WritableMemory;

use crate::interpreter::common::MEM_SIZE_WORDS;

/* Constants */

//TODO

/* Macros */

//Matching address patterns for various memories and MMIO devices
macro_rules! WORK_RAM_ADDR { () => {0x000000..=0x0027FF} }
macro_rules! RENDER_ADDR { () => {0x002800..=0x002FFF} }
macro_rules! SOUND_ADDR { () => {0x003000..=0x0037FF} }
macro_rules! IO_ADDR { () => {0x003D00..=0x003D20 | 0x003D24..=0x003D2E | 0x003D30..=0x3DFF} }
macro_rules! DMA_ADDR { () => {0x003E00..=0x003E03} }
macro_rules! BIOS_ADDR { () => {0x004000..=0x0FFFFF} }
macro_rules! CARTRIDGE_ADDR { () => {0x100000..=0x3FFFFF} }
macro_rules! INT_CTRL_REG { () => {0x003D21} }
macro_rules! INT_CLEAR_REG { () => {0x003D22} }
macro_rules! EXTMEM_REG_ADDR { () => {0x003D23} }
macro_rules! DS_REG_ADDR { () => {0x003D2F}}

/* Static Variables */

//TODO

/* Types */


impl InstructionMemory for State {
    fn should_invalidate_icache(self: &Self) -> bool {
        todo!();
    }

    fn fetch_addr(self: &Self, addr: u32) -> u16 {//For instruction fetching only (faster)
        log_ansi!(1, "\x1b[33m", "(Mem Access: Fetch from address {:#08X})", addr);
        debug_assert!((addr as usize) <= MEM_SIZE_WORDS);

        let data: u16;

        match addr {
            BIOS_ADDR!() => { data = self.bios.fetch_addr(addr); },
            CARTRIDGE_ADDR!() => { data = self.cartridge.fetch_addr(addr); },
            _ => { return debug_panic!(0); },//Invalid address, access to unallocated address space, or to non-instruction memory
        }

        log_ansi!(1, "\x1b[33m", "(Mem Access: Fetch {:#06X})", data);
        return data;
    }
}

impl ReadableMemory for State {
    fn read_addr(self: &Self, addr: u32) -> u16 {
        log_ansi!(1, "\x1b[32m", "(Mem Access: Read from address {:#08X})", addr);
        debug_assert!((addr as usize) <= MEM_SIZE_WORDS);

        let data: u16;

        match addr {
            WORK_RAM_ADDR!() => { log!(2, "Work Ram"); data = self.work_ram[addr as usize]; },
            RENDER_ADDR!() => { data = self.render.read_addr(addr); },
            SOUND_ADDR!() => { data = self.sound.read_addr(addr); },
            IO_ADDR!() => { data = self.io.read_addr(addr); },
            DMA_ADDR!() => { todo!(); },
            BIOS_ADDR!() => { data = self.bios.read_addr(addr); },
            CARTRIDGE_ADDR!() | EXTMEM_REG_ADDR!() => { data = self.cartridge.read_addr(addr); },//TODO split these into two cases for efficiency and add function to cartridge for accessing the reg
            DS_REG_ADDR!() => { log!(2, "DS field in SR register"); data = self.get_ds() as u16; },
            INT_CTRL_REG!() => { log!(2, "INT_CTRL register"); data = self.int_ctrl_reg; },
            INT_CLEAR_REG!() => { log!(2, "INT_CLEAR register"); data = 0xDEAD; },//TODO this will likely need to be a hook to clear interrupts in structures in State
            _ => { return debug_panic!(0); },//Invalid address or access to unallocated address space
        }

        log_ansi!(1, "\x1b[32m", "(State Mem Access: Read {:#06X})", data);
        return data;
    }
}

impl WritableMemory for State {
    fn write_addr(self: &mut Self, addr: u32, data: u16) {
        log_ansi!(1, "\x1b[35m", "(Mem Access: Write {:#06X} to address {:#08X})", data, addr);
        debug_assert!((addr as usize) <= MEM_SIZE_WORDS);

        match addr {
            WORK_RAM_ADDR!() => { log!(2, "Work Ram"); self.work_ram[addr as usize] = data; },
            RENDER_ADDR!() => { self.render.write_addr(addr, data); },
            SOUND_ADDR!() => { self.sound.write_addr(addr, data); },
            IO_ADDR!() => { self.io.write_addr(addr, data); },
            DMA_ADDR!() => { todo!(); },
            CARTRIDGE_ADDR!() | EXTMEM_REG_ADDR!() => { self.cartridge.write_addr(addr, data); },//TODO split these into two cases for efficiency and add function to cartridge for accessing the reg
            DS_REG_ADDR!() => { log!(2, "DS field in SR register"); self.set_ds((data & 0b111111) as u8); },
            INT_CTRL_REG!() => { log!(2, "INT_CTRL register"); self.int_ctrl_reg = data; },
            INT_CLEAR_REG!() => { log!(2, "INT_CLEAR register"); },//TODO this will likely need to be a hook to clear interrupts in structures in State
            _ => { debug_panic!(); },//Invalid address or access to unallocated address space
        }

        log_ansi!(1, "\x1b[35m", "(Mem Access: Write finished)");
    }
}

/* Associated Functions and Methods */

//TODO

/* Functions */

//TODO
