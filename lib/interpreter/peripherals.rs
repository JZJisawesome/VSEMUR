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

use super::render;
use super::sound;
use super::io;
use super::memory;

/* Constants */

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
const BIOS_BEGIN_ADDR: u32 = 0x003E04;//TODO figure out what this is
const BIOS_END_ADDR: u32 = 0x0FFFFF;//TODO figure out what this is
const ROM_BEGIN_ADDR: u32 = 0x100000;//TODO figure out what this is
const ROM_END_ADDR: u32 = 0x3FFFFF;//TODO figure out what this is

/* Macros */

//TODO (also pub(crate) use the_macro statements here too)

/* Static Variables */

//TODO

/* Types */

pub(super) struct Peripherals {
    render: render::RenderState,
    sound: sound::SoundState,
    io: io::IOState,
    mem: memory::MemoryState,
}

/* Associated Functions and Methods */

impl Peripherals {
    //TODO it will have a bunch of weak references to memory-mapped structs in State
    //Or perhaps just have a function to "attach memory device" taking a struct implementing the Memory trait
    pub(super) fn new() -> Peripherals {
        log!(1, "Initializing peripherals");

        //TODO implement
        return Peripherals {
            render: render::RenderState::new(),
            sound: sound::SoundState::new(),
            io: io::IOState::new(),
            mem: memory::MemoryState::new(),
        };
    }

    pub(super) fn reset(self: &mut Self) {
        log!(1, "Resetting peripherals");

        self.mem.reset();
        self.render.reset(&mut self.mem);
        self.sound.reset();
        self.io.reset(&mut self.mem);
    }

    pub fn tick(self: &mut Self) {
        //unimplemented!();//TODO
        //TESTING
        //log!(1, "Printing render register contents");
        //for i in 0x002800..=0x0028FF {
        //    log!(2, "{:#08X}: {:#06X}", i, mem.read_addr(i));
        //}
    }
}

impl Memory for Peripherals {
    fn read_addr(self: &Self, addr: u32) -> u16 {
        debug_assert!((addr as usize) <= MEM_SIZE_WORDS);

        if (addr >= 0x2800) && (addr <= 0x7FFF) {//TESTING
            log_ansi!(0, "\x1b[31m", "Read from location outside of memory or bios/rom: {:#06X}", addr);
        }
        //TODO for now we only read from memory
        return self.mem.read_addr(addr);
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
    }

    fn write_addr(self: &mut Self, addr: u32, data: u16) {
        debug_assert!((addr as usize) <= MEM_SIZE_WORDS);

                if addr >= 0x2800 {//TESTING
            log_ansi!(0, "\x1b[31m", "Write to location outside of memory: {:#06X}", addr);
        }

        //TODO for now we only write to memory
        return self.mem.write_addr(data, addr);

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
    }
}

/* Functions */

//TODO
