/* memory.rs
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
use crate::logging::log_ansi;
use crate::interpreter::ReturnCode;
use crate::interpreter::MAX_BIOS_SIZE_WORDS;
use crate::interpreter::MAX_ROM_SIZE_WORDS;
use crate::interpreter::MEM_SIZE_WORDS;//TODO set this to 0xFFFF since everything above this should not be writable

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
        log!(1, "Initializing memory");
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
        let load_result = load_file_u16(path, &mut self.bios, MAX_BIOS_SIZE_WORDS);
        if matches!(load_result, ReturnCode::LoadOk) {
            self.bios_loaded = true;
        }
        return load_result;
    }

    pub(super) fn load_bios_mem(self: &mut Self, bios_mem: &[u16]) -> ReturnCode {
        if bios_mem.len() > MAX_BIOS_SIZE_WORDS {
            return ReturnCode::LoadFailSize;
        }
        //TODO bios_mem copy into self.bios
        self.bios_loaded = true;
        unimplemented!();
        return ReturnCode::LoadOk;
    }

    pub(super) fn load_rom_file(self: &mut Self, path: &str) -> ReturnCode {
        let load_result = load_file_u16(path, &mut self.rom, MAX_ROM_SIZE_WORDS);
        if matches!(load_result, ReturnCode::LoadOk) {
            self.rom_loaded = true;
        }
        return load_result;
    }

    pub(super) fn load_rom_mem(self: &mut Self, rom_mem: &[u16]) -> ReturnCode {
        if rom_mem.len() > MAX_ROM_SIZE_WORDS {
            return ReturnCode::LoadFailSize;
        }
        //TODO rom_mem copy into self.rom
        self.rom_loaded = true;
        unimplemented!();
        return ReturnCode::LoadOk;
    }

    pub(super) fn reset(self: &mut Self) -> bool {
        if !self.bios_loaded || !self.rom_loaded {
            return false;
        }

        log!(1, "Resetting memory");

        log!(2, "Place loaded bios and rom into the address space");
        //TODO
        //TEMPORARY for now just copy the bios to the memory
        self.mem.clone_from(&self.bios);

        //TODO registers/etc?

        self.mem_loaded = true;
        return true;
    }

    pub(super) fn ready(self: &Self) -> bool {
        return self.bios_loaded && self.rom_loaded && self.mem_loaded;
    }

    //TODO memory access functions (will need to implement the memory map of the processor)
    pub(super) fn read_addr(self: &Self, addr: u32) -> u16 {
        debug_assert!((addr as usize) < MEM_SIZE_WORDS);

        debug_assert!((addr <= 0x3e03) || (addr >= 0x8000));//According to MAME, this is the highest address of a peripheral in the system; 0x8000 is the start of the bios
        return self.mem[addr as usize];
    }

    //TODO memory access functions
    pub(super) fn read_page_addr(self: &Self, page: u8, addr: u16) -> u16 {
        return self.read_addr(((page as u32) << 16) | (addr as u32));
    }

    pub(super) fn write_addr(self: &mut Self, data: u16, addr: u32) {
        debug_assert!((addr as usize) < MEM_SIZE_WORDS);
        debug_assert!(addr <= 0x3e03);//According to MAME, this is the highest address of a peripheral in the system
        self.mem[addr as usize] = data;
    }

    pub(super) fn write_page_addr(self: &mut Self, data: u16, page: u8, addr: u16) {
        return self.write_addr(data, ((page as u32) << 16) | (addr as u32));
    }
}

/* Functions */

fn load_file_u16(path: &str, buffer: &mut [u16], buffer_size: usize) -> ReturnCode {
    //Open the file
    let file_wrapper = File::open(path);
    if matches!(file_wrapper, Err(_)) {
        return ReturnCode::LoadFailOpen;
    }
    let mut file = file_wrapper.unwrap();

    //Ensure it is not larger than expected
    let metadata_wrapper = file.metadata();
    if matches!(metadata_wrapper, Err(_)) {
        return ReturnCode::LoadFailOpen;
    }
    let metadata = metadata_wrapper.unwrap();
    if metadata.len() > (buffer_size * 2) as u64 {//Ensure it is not too big of a file
        return ReturnCode::LoadFailSize;
    }
    if (metadata.len() & 0b1) == 0b1 {//Ensure the file is a multiple of 2
        return ReturnCode::LoadFailSize;
    }

    log_ansi!(0, "\x1b[36m", "Loading file \"{}\": {} words | {} bytes", path, metadata.len() / 2, metadata.len());

    //Read in its contents into the buffer
    let mut byte_buffer: Box<[u8]> = vec![0u8; buffer_size * 2].into_boxed_slice();//TODO avoid overhead of zeroing out contents, as well as overhead of needing to copy to buffer instead of reading to it directly
    let bytes_read = file.read(&mut byte_buffer).unwrap();
    debug_assert!(bytes_read <= buffer_size * 2);

    //Files are little-endian
    for i in 0..buffer_size {//FIXME this loop is incredibly slow
        buffer[i] = ((byte_buffer[(i * 2) + 1] as u16) << 8) | (byte_buffer[i * 2] as u16);
    }
    return ReturnCode::LoadOk;
}
