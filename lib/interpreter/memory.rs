/* memory.rs
 * By: John Jekel
 *
 * MemoryState and functions for VSmile emulation
 *
*/

/* Imports */

use crate::logging::log;
use crate::interpreter::common::ReturnCode;
use crate::interpreter::common::MAX_BIOS_SIZE_WORDS;
use crate::interpreter::common::MAX_ROM_SIZE_WORDS;
use crate::interpreter::common::MEM_SIZE_WORDS;

use std::fs::File;
use std::io::Read;

/* Constants */

//TODO

/* Macros */

//TODO (also pub(crate) use the_macro statements here too)

/* Static Variables */

//TODO

/* Types */

pub(super) struct MemoryState {
    bios_loaded: bool,
    rom_loaded: bool,
    mem_loaded: bool,
    bios: Box<[u16]>,
    rom: Box<[u16]>,
    mem: Box<[u16]>,
}

/* Associated Functions and Methods */

impl MemoryState {
    pub(super) fn new() -> MemoryState {
        log!(0, 1, "Initializizing Memory State");
        return MemoryState {
            /*
            //FIXME use this instead once it is stable
            bios: box [0; MAX_BIOS_SIZE_WORDS],//TODO avoid zero-initializing for speed
            rom: box [0; MAX_ROM_SIZE_WORDS],//TODO avoid zero-initializing for speed
            mem: box [0; MEM_SIZE_WORDS],//TODO avoid zero-initializing for speed
            */
            bios_loaded: false,
            rom_loaded: false,
            mem_loaded: false,
            bios: vec![0u16; MAX_BIOS_SIZE_WORDS].into_boxed_slice(),//TODO avoid vector for speed//TODO avoid zero-initializing for speed
            rom: vec![0u16; MAX_ROM_SIZE_WORDS].into_boxed_slice(),//TODO avoid vector for speed//TODO avoid zero-initializing for speed
            mem: vec![0u16; MEM_SIZE_WORDS].into_boxed_slice(),//TODO avoid vector for speed//TODO avoid zero-initializing for speed
        }
    }

    pub(super) fn load_bios_file(self: &mut Self, path: &str) -> ReturnCode {
        let load_result = load_file(path, &mut self.bios, MAX_BIOS_SIZE_WORDS);
        if matches!(load_result, ReturnCode::LOAD_OK) {
            self.bios_loaded = true;
        }
        return load_result;
    }

    pub(super) fn load_bios_mem(self: &mut Self, bios_mem: &[u16]) -> ReturnCode {
        unimplemented!();//TODO implement
    }

    pub(super) fn load_rom_file(self: &mut Self, path: &str) -> ReturnCode {
        let load_result = load_file(path, &mut self.rom, MAX_ROM_SIZE_WORDS);
        if matches!(load_result, ReturnCode::LOAD_OK) {
            self.rom_loaded = true;
        }
        return load_result;
    }

    pub(super) fn load_rom_mem(self: &mut Self, rom_mem: &[u16]) -> ReturnCode {
        if rom_mem.len() > MAX_ROM_SIZE_WORDS {
            return ReturnCode::LOAD_FAIL_SIZE;
        }
        unimplemented!();//TODO rom_mem copy into self.rom
        self.rom_loaded = true;
        return ReturnCode::LOAD_OK;
    }

    pub(super) fn reset(self: &mut Self) -> bool {
        if !self.bios_loaded || !self.rom_loaded {
            return false
        }

        log!(0, 1, "Resetting memory");

        //TODO
        //TEMPORARY for now just copy the bios to the memory
        self.mem.clone_from(&self.bios);

        self.mem_loaded = true;
        return true;
    }

    pub(super) fn ready(self: &mut Self) -> bool {
        return self.bios_loaded && self.rom_loaded && self.mem_loaded;
    }

    //TODO memory access functions
}

/* Functions */

fn load_file(path: &str, buffer: &mut [u16], buffer_size: usize) -> ReturnCode {
    //Open the file
    let file_wrapper = File::open(path);
    if matches!(file_wrapper, Err(_)) {
        return ReturnCode::LOAD_FAIL_OPEN;
    }
    let mut file = file_wrapper.unwrap();

    //Ensure it is not larger than expected
    let metadata_wrapper = file.metadata();
    if matches!(metadata_wrapper, Err(_)) {
        return ReturnCode::LOAD_FAIL_OPEN;
    }
    let metadata = metadata_wrapper.unwrap();
    if metadata.len() > (buffer_size * 2) as u64 {//Ensure it is not too big of a file
        return ReturnCode::LOAD_FAIL_SIZE;
    }
    if (metadata.len() & 0b1) == 0b1 {//Ensure the file is a multiple of 2
        return ReturnCode::LOAD_FAIL_SIZE;
    }

    log!(0, 0, "Loading file \"{}\": {} words | {} bytes", path, metadata.len() / 2, metadata.len());

    //Read in its contents into the buffer
    let mut byte_buffer: Box<[u8]> = vec![0u8; buffer_size * 2].into_boxed_slice();//TODO avoid overhead of zeroing out contents, as well as overhead of needing to copy to buffer instead of reading to it directly
    file.read(&mut byte_buffer);
    for i in 0..buffer_size {
        buffer[i] = ((byte_buffer[(i * 2) + 1] as u16) << 8) | (byte_buffer[i * 2] as u16);
    }
    return ReturnCode::LOAD_OK;
}
