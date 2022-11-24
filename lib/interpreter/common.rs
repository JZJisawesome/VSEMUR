/* NAME//TODO
 * By: John Jekel
 *
 * TODO description
 *
*/

/* Imports */

//TODO (include "use" and "mod" here)

/* Constants */

pub(super) const MAX_BIOS_SIZE_WORDS: usize = 1 << 22;//FIXME figure out what this actually is
pub(super) const MAX_ROM_SIZE_WORDS: usize = 1 << 22;//FIXME figure out what this actually is
pub(super) const MEM_SIZE_WORDS: usize = 1 << 22;
pub(super) const PHYSICAL_MEM_SIZE_WORDS: usize = 1024 * 10;//10 kilowords of memory

/* Macros */

//TODO (also pub(crate) use the_macro statements here too)

/* Static Variables */

//TODO

/* Types */

pub(super) enum Interrupt {
    Break,
    FIQ,
    Reset,
    IRQ0,
    IRQ1,
    IRQ2,
    IRQ3,
    IRQ4,
    IRQ5,
    IRQ6,
    IRQ7
}

pub(super) trait CPU {
    fn reg_sp(self: &mut Self) -> &mut u16;
    fn reg_r(self: &mut Self) -> &mut [u16;4];
    fn reg_bp(self: &mut Self) -> &mut u16;
    fn reg_sr(self: &mut Self) -> &mut u16;
    fn reg_pc(self: &mut Self) -> &mut u16;
    fn reg_fr(self: &mut Self) -> &mut u16;

    //TODO we need to also acknowledge we finished an interrupt too
    fn soft_interrupt_request(self: &mut Self);//To support the BREAK instruction

    //TODO add helper functions in cpu.rs to this trait as defaults
}

pub(super) trait Tickable {
    fn tick(self: &mut Self) -> bool;//Returns true if an interrupt is requested
}

pub(super) trait InterruptReadable {//Used by handle_interrupts
    fn get_interrupt(self: &mut Self) -> Option<Interrupt>;
}

pub(super) trait InterruptClearable {//Used by execute_inst to acknowledge when we've finished with an interrupt
    fn clear_current_interrupt(self: &mut Self);
}

pub(super) trait InstructionMemory: ReadableMemory {
    fn should_invalidate_icache(self: &Self) -> bool;//Useful for caching interpretation

    fn fetch_addr(self: &Self, addr: u32) -> u16 {
        return self.read_addr(addr);
    }

    fn fetch_page_addr(self: &Self, page: u8, addr: u16) -> u16 {
        return self.fetch_addr(((page as u32) << 16) | (addr as u32));
    }
}

pub(super) trait ReadableMemory {
    fn read_addr(self: &Self, addr: u32) -> u16;

    fn read_page_addr(self: &Self, page: u8, addr: u16) -> u16 {
        return self.read_addr(((page as u32) << 16) | (addr as u32));
    }
}

pub(super) trait WritableMemory {
    fn write_addr(self: &mut Self, addr: u32, data: u16);

    fn write_page_addr(self: &mut Self, page: u8, addr: u16, data: u16) {
        self.write_addr(((page as u32) << 16) | (addr as u32), data);
    }
}

/* Associated Functions and Methods */

//TODO

/* Functions */

pub(super) fn load_file_u16(path: &str, buffer: &mut [u16]) -> Result<(), ()> {
    use crate::logging::log_ansi;
    use std::fs::File;
    use std::io::Read;

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
