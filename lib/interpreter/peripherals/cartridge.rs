/* NAME//TODO
 * By: John Jekel
 *
 * TODO description
 *
*/

/* Imports */

use crate::debug_panic;

use crate::logging::log;

use crate::interpreter::common::MAX_ROM_SIZE_WORDS;
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

pub(super) struct Cartridge {
    rom_loaded: bool,
    rom: Option<Box<[u16]>>,//Option because we don't require a ROM for things to work, so we can save memory
    reg_ext_memory_ctrl: u16,
}

/* Associated Functions and Methods */

impl Cartridge {
    pub(super) fn new() -> Cartridge {
        log!(2, "Initializing cartridge state");
        return Cartridge {
            rom_loaded: false,
            rom: None,
            reg_ext_memory_ctrl: 0x0028,
        };
    }

    pub(super) fn load_file(self: &mut Self, path: &str) -> Result<(), ()> {
        //Allocate space for the ROM if we haven't yet
        //By waiting to do this until now we save memory if the user wants to start the VSmile without a ROM
        if matches!(self.rom, None) {
            //TODO avoid vector for speed
            //TODO avoid zero-initializing for speed
            //TODO perhaps only allocate the memory necessary?
            self.rom.replace(vec![0u16; MAX_ROM_SIZE_WORDS].into_boxed_slice());
        }
        let result = load_file_u16(path, self.rom.as_mut().unwrap());
        if matches!(result, Ok(())) {
            self.rom_loaded = true;
        }
        return result;
    }

    pub(super) fn load_mem(self: &mut Self, rom_mem: &[u16]) -> Result<(), ()> {
        todo!();
    }

    //TODO functions to save the NVRAM to disk
}

impl InstructionMemory for Cartridge {
    fn should_invalidate_icache(self: &Self) -> bool {
        return false;//TODO unless we switch banks!
    }
}

impl Memory for Cartridge {
    fn read_addr(self: &Self, addr: u32) -> u16 {
        //TODO do this properly (actually support a rom + bank switching, proper memory regions for bios vs rom)
        if addr == 0x003D23 {
            log!(2, "Cartridge Access (REG_EXT_MEMORY_CTRL)");
            return self.reg_ext_memory_ctrl;
        } else {
            todo!();//Different message in brackets based on whether this is the ROM or the NVRAM
        }
    }

    fn write_addr(self: &mut Self, addr: u32, data: u16) {
        //TODO what about NVRAM?
        debug_assert!(addr == 0x003D23);//The only write we should be recieving is to REG_EXT_MEMORY_CTRL
        log!(2, "Cartridge Access (REG_EXT_MEMORY_CTRL)");
        self.reg_ext_memory_ctrl = data;
    }
}

/*impl Memory for Cartridge {

}*///TODO

/* Functions */

//TODO
