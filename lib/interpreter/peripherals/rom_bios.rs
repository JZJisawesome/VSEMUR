/* rom_bios.rs
 * By: John Jekel
 *
 * Manages the state for read-only memories of the VSmile.
 * This includes its BIOS and ROMs
 *
*/

/* Imports */

use crate::debug_panic;

use crate::logging::log;
use crate::logging::log_ansi;

use crate::interpreter::common::MAX_BIOS_SIZE_WORDS;
use crate::interpreter::common::MAX_ROM_SIZE_WORDS;
use crate::interpreter::common::Memory;

use std::fs::File;
use std::io::Read;

/* Constants */

//TODO

/* Macros */

//TODO (also pub(crate) use the_macro statements here too)

/* Static Variables */

//TODO

/* Types */

pub(super) struct RomAndBiosState {
    bios_loaded: bool,
    rom_loaded: bool,
    bios: Box<[u16]>,
    rom: Option<Box<[u16]>>,//Option because we don't require a ROM for things to work, so we can save memory
}

/* Associated Functions and Methods */

impl RomAndBiosState {
    pub(super) fn new() -> RomAndBiosState {
        log!(2, "Initializing ROM/BIOS-related state");
        return RomAndBiosState {
            bios_loaded: false,
            rom_loaded: false,
            bios: vec![0u16; MAX_BIOS_SIZE_WORDS].into_boxed_slice(),//TODO avoid vector for speed//TODO avoid zero-initializing for speed//TODO perhaps only allocate the memory necessary?
            rom: None,
        };
    }

    pub(super) fn load_bios_file(self: &mut Self, path: &str) -> Result<(), ()> {
        let result = load_file_u16(path, &mut self.bios);
        if matches!(result, Ok(())) {
            self.bios_loaded = true;
        }
        return result;
    }

    pub(super) fn load_bios_mem(self: &mut Self, bios_mem: &[u16]) -> Result<(), ()> {
        todo!();
    }

    pub(super) fn load_rom_file(self: &mut Self, path: &str) -> Result<(), ()> {
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

    pub(super) fn load_rom_mem(self: &mut Self, rom_mem: &[u16]) -> Result<(), ()> {
        todo!();
    }

    pub(super) fn ready(self: &Self) -> bool {
        return self.bios_loaded;//The ROM does not need to be loaded for us to be ready
    }
}

impl Memory for RomAndBiosState {
    fn read_addr(self: &Self, addr: u32) -> u16 {
        //TODO do this properly (actually support a rom + bank switching, proper memory regions for bios vs rom)
        return self.bios[addr as usize];
    }

    fn write_addr(self: &mut Self, _: u32, _: u16) {
        debug_panic!();//We should never be writing to the BIOS/ROM//TODO should we be handling this nicer?
    }
}

/* Functions */

fn load_file_u16(path: &str, buffer: &mut [u16]) -> Result<(), ()> {
    //Open the file
    let file_wrapper = File::open(path);
    if matches!(file_wrapper, Err(_)) {
        return Err(());
    }
    let mut file = file_wrapper.unwrap();

    //Ensure it is not larger than expected
    let metadata_wrapper = file.metadata();
    if matches!(metadata_wrapper, Err(_)) {
        return Err(());
    }
    let metadata = metadata_wrapper.unwrap();
    if metadata.len() > (buffer.len() * 2) as u64 {//Ensure it is not too big of a file
        return Err(());
    }
    if (metadata.len() & 0b1) == 0b1 {//Ensure the file is a multiple of 2
        return Err(());
    }

    log_ansi!(0, "\x1b[36m", "Loading file \"{}\": {} words | {} bytes", path, metadata.len() / 2, metadata.len());

    //Read in its contents into the buffer
    let mut byte_buffer: Box<[u8]> = vec![0u8; buffer.len() * 2].into_boxed_slice();//TODO avoid overhead of zeroing out contents, as well as overhead of needing to copy to buffer instead of reading to it directly
    let bytes_read = file.read(&mut byte_buffer).unwrap();
    debug_assert!(bytes_read <= buffer.len() * 2);

    //Files are little-endian
    for i in 0..buffer.len() {//FIXME this loop is incredibly slow
        buffer[i] = ((byte_buffer[(i * 2) + 1] as u16) << 8) | (byte_buffer[i * 2] as u16);
    }
    return Ok(());
}
