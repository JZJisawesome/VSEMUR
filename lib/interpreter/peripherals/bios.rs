/* NAME//TODO
 * By: John Jekel
 *
 * TODO description
 *
*/

/* Imports */

use crate::debug_panic;

use crate::logging::log;

use crate::interpreter::common::MAX_BIOS_SIZE_WORDS;
use crate::interpreter::common::InstructionMemory;
use crate::interpreter::common::Memory;
use crate::interpreter::common::load_file_u16;

/* Constants */

//TODO

/* Macros */

//TODO (also pub(crate) use the_macro statements here too)

/* Static Variables */

//TODO

/* Types */

pub(super) struct Bios {
    bios_loaded: bool,
    bios: Box<[u16]>,
}

/* Associated Functions and Methods */

impl Bios {
    pub(super) fn new() -> Bios {
        log!(2, "Initializing BIOS-related state");
        return Bios {
            bios_loaded: false,
            bios: vec![0u16; MAX_BIOS_SIZE_WORDS].into_boxed_slice(),//TODO avoid vector for speed//TODO avoid zero-initializing for speed//TODO perhaps only allocate the memory necessary?
        };
    }

    pub(super) fn load_file(self: &mut Self, path: &str) -> Result<(), ()> {
        let result = load_file_u16(path, &mut self.bios);//TODO we only really need to load the part of the file from 0x004000 to 0x0FFFFF
        if matches!(result, Ok(())) {
            self.bios_loaded = true;
        }
        return result;
    }

    pub(super) fn load_mem(self: &mut Self, bios_mem: &[u16]) -> Result<(), ()> {
        todo!();
    }
}

impl InstructionMemory for Bios {
    fn should_invalidate_icache(self: &Self) -> bool {
        return false;//The BIOS dosn't do bank switching, so this is never needed
    }
}

impl Memory for Bios {
    fn read_addr(self: &Self, addr: u32) -> u16 {
        log!(2, "BIOS Access");
        debug_assert!(self.bios_loaded);
        //TODO error checking addr
        return self.bios[addr as usize];
    }

    fn write_addr(self: &mut Self, addr: u32, data: u16) {
        log!(2, "BIOS Access");
        debug_panic!();
    }
}

/* Functions */

//TODO
