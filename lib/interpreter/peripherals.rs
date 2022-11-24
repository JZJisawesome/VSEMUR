/* peripherals.rs
 * By: John Jekel
 *
 * Emulation of the peripherals of the VSmile's SOC, both internal and external
 *
 * Implementes the Memory trait, so everything is memory-mapped for the CPU's use
 *
*/

/* Imports */

use crate::debug_panic;

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

//TODO

/* Macros */

//Matching address patterns for various peripherals
macro_rules! WORK_RAM_ADDR { () => {0x000000..=0x0027FF} }
macro_rules! RENDER_ADDR { () => {0x002800..=0x002FFF} }
macro_rules! SOUND_ADDR { () => {0x003000..=0x0037FF} }
//macro_rules! IO_ADDR { () => {0x003D00..=0x3DFF} }
macro_rules! IO_ADDR { () => {0x003D00..=0x003D22 | 0x003D24..=0x3DFF} }
macro_rules! DMA_ADDR { () => {0x003E00..=0x003E03} }
macro_rules! BIOS_ADDR { () => {0x004000..=0x0FFFFF} }
//macro_rules! CARTRIDGE_ADDR { () => {0x100000..=0x3FFFFF} }
macro_rules! CARTRIDGE_ADDR { () => {0x100000..=0x3FFFFF | 0x003D23} }

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
        log!(1, "Peripherals: Tick begins");
        //todo!();//TODO
        log!(1, "Peripherals: Tick ends");
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

        match addr {
            WORK_RAM_ADDR!() => { data = self.work_ram[addr as usize]; },
            RENDER_ADDR!() => { todo!(); },
            SOUND_ADDR!() => { todo!(); },
            IO_ADDR!() => { data = self.io.read_addr(addr); },
            DMA_ADDR!() => { todo!(); },
            BIOS_ADDR!() => { data = self.rom_bios.read_addr(addr); },
            CARTRIDGE_ADDR!() => { data = self.rom_bios.read_addr(addr); },
            _ => { return debug_panic!(0); },//Invalid address or access to unallocated address space
        }

        log_ansi!(1, "\x1b[32m", "(Peripherals Mem Access: Read {:#06X})", data);
        return data;
    }

    fn write_addr(self: &mut Self, addr: u32, data: u16) {
        debug_assert!((addr as usize) <= MEM_SIZE_WORDS);
        log_ansi!(1, "\x1b[35m", "(Peripherals Mem Access: Write {:#06X} to address {:#08X})", data, addr);

        match addr {
            WORK_RAM_ADDR!() => { self.work_ram[addr as usize] = data; },
            RENDER_ADDR!() => { todo!(); },
            SOUND_ADDR!() => { todo!(); },
            IO_ADDR!() => { self.io.write_addr(addr, data); },
            DMA_ADDR!() => { todo!(); },
            BIOS_ADDR!() => { todo!(); },
            CARTRIDGE_ADDR!() => { self.rom_bios.write_addr(addr, data); },
            _ => { debug_panic!(); },//Invalid address or access to unallocated address space
        }

        log_ansi!(1, "\x1b[35m", "(Peripherals Mem Access: Write finished)");
    }
}

/* Functions */

//TODO
