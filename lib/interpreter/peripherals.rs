/* peripherals.rs
 * By: John Jekel
 *
 * Emulation of the peripherals of the VSmile's SOC, both internal and external
 *
 * Implementes the Memory trait, so everything is memory-mapped for the CPU's use
 *
*/

/* Imports */

use crate::logging::log;
use crate::logging::log_ansi;

use super::common::Memory;
use super::common::MEM_SIZE_WORDS;
use crate::interpreter::common::PHYSICAL_MEM_SIZE_WORDS;

use super::render_reciever::RenderReciever;
use super::sound_reciever::SoundReciever;
use super::input_sender::InputSender;

mod io;
mod render;
mod sound;
mod rom_bios;

/* Constants */

//All inclusive
const WORK_RAM_BEGIN_ADDR: u32 = 0x000000;
const WORK_RAM_END_ADDR: u32 = 0x0027FF;
const RENDER_BEGIN_ADDR: u32 = 0x002800;
const RENDER_END_ADDR: u32 = 0x002FFF;
const SOUND_BEGIN_ADDR: u32 = 0x003000;
const SOUND_END_ADDR: u32 = 0x0037FF;
const IO_BEGIN_ADDR: u32 = 0x003D00;
const IO_END_ADDR: u32 = 0x003DFF;
const DMA_BEGIN_ADDR: u32 = 0x003E00;
const DMA_END_ADDR: u32 = 0x003E03;
const BIOS_BEGIN_ADDR: u32 = 0x004000;
const BIOS_END_ADDR: u32 = 0x0FFFFF;
const ROM_BEGIN_ADDR: u32 = 0x100000;
const ROM_END_ADDR: u32 = 0x3FFFFF;

/* Macros */

//TODO (also pub(crate) use the_macro statements here too)

/* Static Variables */

//TODO

/* Types */

pub(super) struct Peripherals {
    render: render::RenderState,
    sound: sound::SoundState,
    io: io::IOState,
    work_ram: Box<[u16]>,
    rom_bios: rom_bios::RomAndBiosState,//TODO split this into two seperate parts
}

/* Associated Functions and Methods */

impl Peripherals {
    //TODO it will have a bunch of weak references to memory-mapped structs in State
    //Or perhaps just have a function to "attach memory device" taking a struct implementing the Memory trait
    pub(super) fn new() -> Peripherals {
        log!(1, "Initializing memory-mapped peripherals");

        //TODO implement
        return Peripherals {
            render: render::RenderState::new(),
            sound: sound::SoundState::new(),
            io: io::IOState::new(),
            work_ram: vec![0u16; PHYSICAL_MEM_SIZE_WORDS].into_boxed_slice(),//TODO avoid vector for speed//TODO avoid zero-initializing for speed//TODO perhaps only allocate the memory necessary?
            rom_bios: rom_bios::RomAndBiosState::new(),
        };
    }

    pub(super) fn reset(self: &mut Self) {
        log!(1, "Resetting peripherals");

        self.render.reset();
        self.sound.reset();
        self.io.reset();
    }

    pub fn tick(self: &mut Self) {
        //todo!();//TODO
    }

    pub fn frame_ended(self: &mut Self) -> bool {
        return false;//TODO
    }

    pub fn get_render_reciever(self: &mut Self) -> RenderReciever {
        todo!();
    }

    pub fn get_sound_reciever(self: &mut Self) -> SoundReciever {
        todo!();
    }

    pub fn get_input_sender(self: &mut Self) -> InputSender {
        todo!();
    }

    pub fn load_bios_file(self: &mut Self, path: &str) -> Result<(), ()> {
        return self.rom_bios.load_bios_file(path);
    }

    pub fn load_bios_mem(self: &mut Self, bios_mem: &[u16]) -> Result<(), ()> {
        return self.rom_bios.load_bios_mem(bios_mem);
    }

    pub fn load_rom_file(self: &mut Self, path: &str) -> Result<(), ()> {
        return self.rom_bios.load_rom_file(path);
    }

    pub fn load_rom_mem(self: &mut Self, rom_mem: &[u16]) -> Result<(), ()> {
        return self.rom_bios.load_rom_mem(rom_mem);
    }
}

impl Memory for Peripherals {
    fn read_addr(self: &Self, addr: u32) -> u16 {
        debug_assert!((addr as usize) <= MEM_SIZE_WORDS);
        log_ansi!(1, "\x1b[32m", "(Peripherals Mem Access: Read from address {:#08X})", addr);

        let data: u16;

        if (addr >= 0x2800) && (addr <= 0x7FFF) {//TESTING
            log_ansi!(2, "\x1b[31m", "Read from location outside of memory or bios/rom: {:#08X}", addr);
        }

        //TODO proper memory map
        if addr < 0x2800 {
            log!(2, "Work ram");
            data = self.work_ram[addr as usize];
        } else if addr > 0x7FFF {
            data = self.rom_bios.read_addr(addr);
        } else if (addr >= IO_BEGIN_ADDR) && (addr <= IO_END_ADDR) {
            data = self.io.read_addr(addr);
        } else {
            todo!();
        }
        /*match addr {
            WORK_RAM_BEGIN_ADDR..=WORK_RAM_END_ADDR => { todo!(); },
            RENDER_BEGIN_ADDR..=RENDER_END_ADDR => { todo!(); },
            SOUND_BEGIN_ADDR..=SOUND_END_ADDR => { todo!(); },
            IO_BEGIN_ADDR..=IO_END_ADDR => { todo!(); },
            DMA_BEGIN_ADDR..=DMA_END_ADDR => { todo!(); },
            BIOS_BEGIN_ADDR..=BIOS_END_ADDR => { todo!(); },
            ROM_BEGIN_ADDR..=ROM_END_ADDR => { todo!(); },
            _ => { return debug_panic!(0); },//Invalid address or access to unallocated address space
        }
        */

        log_ansi!(1, "\x1b[32m", "(Peripherals Mem Access: Read {:#06X})", data);
        return data;
    }

    fn write_addr(self: &mut Self, addr: u32, data: u16) {
        debug_assert!((addr as usize) <= MEM_SIZE_WORDS);
        log_ansi!(1, "\x1b[35m", "(Peripherals Mem Access: Write {:#06X} to address {:#08X})", data, addr);

        if addr >= 0x2800 {//TESTING
            log_ansi!(2, "\x1b[31m", "Write to location outside of memory: {:#08X}", addr);
        }

        //TODO for now we only write to memory
        //TODO proper memory map
        if addr < 0x2800 {
            log!(2, "Work ram");
            self.work_ram[addr as usize] = data;
        } else if (addr >= IO_BEGIN_ADDR) && (addr <= IO_END_ADDR) {
            self.io.write_addr(addr, data);
        } else {
            todo!();
        }

        /*match addr {
            WORK_RAM_BEGIN_ADDR..=WORK_RAM_END_ADDR => { todo!(); },
            RENDER_BEGIN_ADDR..=RENDER_END_ADDR => { todo!(); },
            SOUND_BEGIN_ADDR..=SOUND_END_ADDR => { todo!(); },
            IO_BEGIN_ADDR..=IO_END_ADDR => { todo!(); },
            DMA_BEGIN_ADDR..=DMA_END_ADDR => { todo!(); },
            BIOS_BEGIN_ADDR..=BIOS_END_ADDR => { todo!(); },
            ROM_BEGIN_ADDR..=ROM_END_ADDR => { todo!(); },
            _ => { return debug_panic!(0); },//Invalid address or access to unallocated address space
        }
        */
        log_ansi!(1, "\x1b[35m", "(Peripherals Mem Access: Write finished)");
    }
}

/* Functions */

//TODO
