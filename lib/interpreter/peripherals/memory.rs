/* memory.rs//TODO get rid of this module
 * By: John Jekel
 *
 * MemoryState and functions for VSmile emulation
 *
*/

//TODO remove this once everything is implemented
#![allow(unused_variables)]
#![allow(dead_code)]
#![allow(unreachable_code)]

/* Imports */

use crate::logging::log;
use crate::interpreter::common::MEM_SIZE_WORDS;//TODO set this to 0xFFFF since everything above this should not be writable

/* Constants */

//TODO

/* Macros */

//TODO (also pub(crate) use the_macro statements here too)

/* Static Variables */

//TODO

/* Types */

pub(super) struct MemoryState {
    mem: Box<[u16]>,

    //peripherals_accessed_since_last_tick: bool,//TODO come up with a scheme for dealing with peripheral registers
}

/* Associated Functions and Methods */

impl MemoryState {
    pub(super) fn new() -> MemoryState {
        log!(2, "Initializing memory");
        return MemoryState {
            /*
            //FIXME use this instead once it is stable
            bios: box [0; MAX_BIOS_SIZE_WORDS],//TODO avoid zero-initializing for speed
            rom: box [0; MAX_ROM_SIZE_WORDS],//TODO avoid zero-initializing for speed
            mem: box [0; MEM_SIZE_WORDS],//TODO avoid zero-initializing for speed
            */
            mem: vec![0u16; MEM_SIZE_WORDS].into_boxed_slice(),//TODO avoid vector for speed//TODO avoid zero-initializing for speed
        }
    }

    pub(super) fn reset(self: &mut Self) {
        /*if !self.bios_loaded || !self.rom_loaded {
            return false;
        }

        log!(2, "Resetting memory");

        log!(3, "Place loaded bios and rom into the address space");
        //TODO
        //TEMPORARY for now just copy the bios to the memory
        self.mem.clone_from(&self.bios);

        //TODO registers/etc?

        self.mem_loaded = true;
        return true;
        */
    }

    pub(super) fn ready(self: &Self) -> bool {
        return true;
    }

    //TODO memory access functions (will need to implement the memory map of the processor)
    pub(super) fn read_addr(self: &Self, addr: u32) -> u16 {
        debug_assert!((addr as usize) < MEM_SIZE_WORDS);

        //TODO we'll need a match statement here to decide what to do with the read based on the address

        //debug_assert!((addr <= 0x3e03) || (addr >= 0x8000));//According to MAME, this is the highest address of a peripheral in the system; 0x8000 is the start of the bios

        return self.mem[addr as usize];
    }

    //TODO memory access functions
    pub(super) fn read_page_addr(self: &Self, page: u8, addr: u16) -> u16 {
        return self.read_addr(((page as u32) << 16) | (addr as u32));
    }

    pub(super) fn write_addr(self: &mut Self, data: u16, addr: u32) {
        debug_assert!((addr as usize) < MEM_SIZE_WORDS);
        //debug_assert!(addr <= 0x3e03);//According to MAME, this is the highest address of a peripheral in the system

        //TODO we'll need a match statement here to decide what to do with the write based on the address




        self.mem[addr as usize] = data;
    }

    pub(super) fn write_page_addr(self: &mut Self, data: u16, page: u8, addr: u16) {
        return self.write_addr(data, ((page as u32) << 16) | (addr as u32));
    }
}

/* Functions */
