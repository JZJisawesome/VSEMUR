/* NAME//TODO
 * By: John Jekel
 *
 * TODO description
 *
*/

/* Imports */

use crate::debug_panic;
use crate::decode;

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
    IRQ0,
    IRQ1,
    IRQ2,
    IRQ3,
    IRQ4,
    IRQ5,
    IRQ6,
    IRQ7
}

//TODO perhaps move this to a different module?
pub(super) trait CPU {
    fn reg_sp(self: &Self) -> &u16;
    fn reg_r(self: &Self) -> &[u16;4];
    fn reg_bp(self: &Self) -> &u16;
    fn reg_sr(self: &Self) -> &u16;
    fn reg_pc(self: &Self) -> &u16;
    fn reg_fr(self: &Self) -> &u16;
    fn reg_sp_mut(self: &mut Self) -> &mut u16;
    fn reg_r_mut(self: &mut Self) -> &mut [u16;4];
    fn reg_bp_mut(self: &mut Self) -> &mut u16;
    fn reg_sr_mut(self: &mut Self) -> &mut u16;
    fn reg_pc_mut(self: &mut Self) -> &mut u16;
    fn reg_fr_mut(self: &mut Self) -> &mut u16;

    fn soft_interrupt_request(self: &mut Self);//To support the BREAK instruction

    fn inc_pc_by(self: &mut Self, increment_amount: u32) {
        let result: (u8, u16) = inc_page_addr_by(self.get_cs(), *self.reg_pc_mut(), increment_amount);
        self.set_cs(result.0);
        *self.reg_pc_mut() = result.1;
    }

    fn inc_pc(self: &mut Self) {
        self.inc_pc_by(1);
    }

    //Getters and setters using reference functions provided above
    fn get_sp(self: &Self) -> u16 {
        return *self.reg_sp();
    }

    fn get_r(self: &Self, index: u8) -> u16 {
        return self.reg_r()[(index - 1) as usize];
    }

    fn get_bp(self: &Self) -> u16 {
        return *self.reg_bp();
    }

    fn get_sr(self: &Self) -> u16 {
        return *self.reg_sr();
    }

    fn get_pc(self: &Self) -> u16 {
        return *self.reg_pc();
    }

    fn get_fr(self: &Self) -> u16 {
        return *self.reg_fr();
    }

    fn set_sp(self: &mut Self, data: u16) {
        *self.reg_sp_mut() = data;
    }

    fn set_r(self: &mut Self, index: u8, data: u16) {
        self.reg_r_mut()[(index - 1) as usize] = data;
    }

    fn set_bp(self: &mut Self, data: u16) {
        *self.reg_bp_mut() = data;
    }

    fn set_sr(self: &mut Self, data: u16) {
        *self.reg_sr_mut() = data;
    }

    fn set_pc(self: &mut Self, data: u16) {
        *self.reg_pc_mut() = data;
    }

    fn set_fr(self: &mut Self, data: u16) {
        *self.reg_fr_mut() = data;
    }

    //SR getters and setters for sub-fields
    fn get_ds(self: &Self) -> u8 {
        return ((*self.reg_sr() >> 10) & 0b111111) as u8;
    }

    fn get_n(self: &Self) -> bool {
        return ((*self.reg_sr() >> 9) & 0b1) == 0b1;
    }

    fn get_z(self: &Self) -> bool {
        return ((*self.reg_sr() >> 8) & 0b1) == 0b1;
    }

    fn get_s(self: &Self) -> bool {
        return ((*self.reg_sr() >> 7) & 0b1) == 0b1;
    }

    fn get_c(self: &Self) -> bool {
        return ((*self.reg_sr() >> 6) & 0b1) == 0b1;
    }

    fn get_cs(self: &Self) -> u8 {
        return (*self.reg_sr() & 0b111111) as u8;
    }

    fn set_ds(self: &mut Self, value: u8) {
        debug_assert!(value < 0b111111);
        *self.reg_sr_mut() = (*self.reg_sr_mut() & 0b0000001111111111) | ((value as u16) << 10);
    }

    fn set_n(self: &mut Self, value: bool) {
        *self.reg_sr_mut() = (*self.reg_sr_mut() & 0b1111110111111111) | ((if value { 0b1 } else { 0b0 }) << 9);
    }

    fn set_z(self: &mut Self, value: bool) {
        *self.reg_sr_mut() = (*self.reg_sr_mut() & 0b1111111011111111) | ((if value { 0b1 } else { 0b0 }) << 8);
    }

    fn set_s(self: &mut Self, value: bool) {
        *self.reg_sr_mut() = (*self.reg_sr_mut() & 0b1111111101111111) | ((if value { 0b1 } else { 0b0 }) << 7);
    }

    fn set_c(self: &mut Self, value: bool) {
        *self.reg_sr_mut() = (*self.reg_sr_mut() & 0b1111111110111111) | ((if value { 0b1 } else { 0b0 }) << 6);
    }

    fn set_cs(self: &mut Self, value: u8) {
        debug_assert!(value < 0b111111);
        *self.reg_sr_mut() = (*self.reg_sr_mut() & 0b1111111111000000) | (value as u16);
    }

    //FR getters and setters
    fn get_aq(self: &Self) -> bool {
        return ((*self.reg_fr() >> 14) & 0b1) == 0b1;
    }

    fn get_bnk(self: &Self) -> bool {
        return ((*self.reg_fr() >> 13) & 0b1) == 0b1;
    }

    fn get_fra(self: &Self) -> bool {
        return ((*self.reg_fr() >> 12) & 0b1) == 0b1;
    }

    fn get_fir(self: &Self) -> bool {
        return ((*self.reg_fr() >> 11) & 0b1) == 0b1;
    }

    fn get_sb(self: &Self) -> u8 {
        return ((*self.reg_fr() >> 7) & 0b1111) as u8;
    }

    fn get_fiq(self: &Self) -> bool {
        return ((*self.reg_fr() >> 6) & 0b1) == 0b1;
    }

    fn get_irq(self: &Self) -> bool {
        return ((*self.reg_fr() >> 5) & 0b1) == 0b1;
    }

    fn get_ine(self: &Self) -> bool {
        return ((*self.reg_fr() >> 4) & 0b1) == 0b1;
    }

    fn get_pri(self: &Self) -> u8 {
        return (*self.reg_fr() & 0b1111) as u8;
    }

    fn set_aq(self: &mut Self, value: bool) {
        *self.reg_fr_mut() = (*self.reg_fr_mut() & 0b1011111111111111) | ((if value { 0b1 } else { 0b0 }) << 14);
    }

    fn set_bnk(self: &mut Self, value: bool) {
        *self.reg_fr_mut() = (*self.reg_fr_mut() & 0b1101111111111111) | ((if value { 0b1 } else { 0b0 }) << 13);
    }

    fn set_fra(self: &mut Self, value: bool) {
        *self.reg_fr_mut() = (*self.reg_fr_mut() & 0b1110111111111111) | ((if value { 0b1 } else { 0b0 }) << 12);
    }

    fn set_fir(self: &mut Self, value: bool) {
        *self.reg_fr_mut() = (*self.reg_fr_mut() & 0b1111011111111111) | ((if value { 0b1 } else { 0b0 }) << 11);
    }

    fn set_sb(self: &mut Self, value: u8) {
        debug_assert!(value < 0b1111);
        *self.reg_fr_mut() = (*self.reg_fr_mut() & 0b1111100001111111) | ((value as u16) << 7);
    }

    fn set_fiq(self: &mut Self, value: bool) {
        *self.reg_fr_mut() = (*self.reg_fr_mut() & 0b1111111110111111) | ((if value { 0b1 } else { 0b0 }) << 6);
    }

    fn set_irq(self: &mut Self, value: bool) {
        *self.reg_fr_mut() = (*self.reg_fr_mut() & 0b1111111111011111) | ((if value { 0b1 } else { 0b0 }) << 5);
    }

    fn set_ine(self: &mut Self, value: bool) {
        *self.reg_fr_mut() = (*self.reg_fr_mut() & 0b1111111111101111) | ((if value { 0b1 } else { 0b0 }) << 4);
    }

    fn set_pri(self: &mut Self, value: u8) {
        debug_assert!(value < 0b1111);
        *self.reg_fr_mut() = (*self.reg_fr_mut() & 0b1111111111110000) | (value as u16);
    }

    //MR getter and setter
    fn get_mr(self: &Self) -> u32 {
        return ((self.reg_r()[3] as u32) << 16) | (self.reg_r()[3] as u32);
    }

    fn set_mr(self: &mut Self, value: u32) {
        self.reg_r_mut()[3] = ((value >> 16) & 0xFFFF) as u16;
        self.reg_r_mut()[2] = (value & 0xFFFF) as u16;
    }

    //Regular registers
    fn get_reg(self: &Self, reg: decode::DecodedRegister) -> u16 {
        use crate::decode::DecodedRegister::*;
        match reg {
            SP => { return *self.reg_sp(); },
            R1_SR1 => { return self.reg_r()[0]; },
            R2_SR2 => { return self.reg_r()[1]; },
            R3_SR3 => { return self.reg_r()[2]; },
            R4_SR4 => { return self.reg_r()[3]; },
            BP => { return *self.reg_bp(); },
            SR => { return *self.reg_sr(); },
            PC => { return *self.reg_pc(); },

            Invalid => { return debug_panic!(0); }//We shouldn't be passed this
        }
    }

    fn set_reg(self: &mut Self, reg: decode::DecodedRegister, value: u16) {
        use crate::decode::DecodedRegister::*;
        match reg {
            SP => { *self.reg_sp_mut() = value; },
            R1_SR1 => { self.reg_r_mut()[0] = value; },
            R2_SR2 => { self.reg_r_mut()[1] = value; },
            R3_SR3 => { self.reg_r_mut()[2] = value; },
            R4_SR4 => { self.reg_r_mut()[3] = value; },
            BP => { *self.reg_bp_mut() = value; },
            SR => { *self.reg_sr_mut() = value; },
            PC => { *self.reg_pc_mut() = value; },

            Invalid => { debug_panic!(); }//We shouldn't be passed this
        }
    }

    fn get_reg_by_index(self: &Self, reg: u8) -> u16 {
        debug_assert!(reg < 8);
        match reg {
            0b000 => { return *self.reg_sp(); },
            0b001 => { return self.reg_r()[0]; },
            0b010 => { return self.reg_r()[1]; },
            0b011 => { return self.reg_r()[2]; },
            0b100 => { return self.reg_r()[3]; },
            0b101 => { return *self.reg_bp(); },
            0b110 => { return *self.reg_sr(); },
            0b111 => { return *self.reg_pc(); },
            _ => { return debug_panic!(0); },//This should never occur
        }
    }

    fn set_reg_by_index(self: &mut Self, reg: u8, value: u16) {
        debug_assert!(reg < 8);
        match reg {
            0b000 => { *self.reg_sp_mut() = value; },
            0b001 => { self.reg_r_mut()[0] = value; },
            0b010 => { self.reg_r_mut()[1] = value; },
            0b011 => { self.reg_r_mut()[2] = value; },
            0b100 => { self.reg_r_mut()[3] = value; },
            0b101 => { *self.reg_bp_mut() = value; },
            0b110 => { *self.reg_sr_mut() = value; },
            0b111 => { *self.reg_pc_mut() = value; },

            _ => { debug_panic!(); }//We shouldn't be passed this
        }
    }
}

//TODO perhaps unique traits for each peripheral, and then State is just a dumb struct?

pub(super) trait Tickable {
    fn tick(self: &mut Self) -> bool;//Returns true if an interrupt is requested
}

pub(super) trait InterruptReadable {//Used by handle_interrupts
    fn get_interrupt(self: &Self) -> Option<Interrupt>;
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

pub(super) fn inc_page_addr_by(page: u8, addr: u16, increment_amount: u32) -> (u8, u16) {
    //TODO we should just use Wrapping types here
    let mut combined_addr: u64 = ((page as u64) << 16) | (addr as u64);//64 bit so we don't need to worry about overflow if increment_amount is large
    combined_addr += increment_amount as u64;
    return (((combined_addr >> 16) & 0b111111) as u8, (combined_addr & 0xFFFF) as u16);
}

pub(super) fn dec_page_addr_by(page: u8, addr: u16, decrement_amount: u32) -> (u8, u16) {
    //TODO we should just use Wrapping types here
    let mut combined_addr: u64 = ((page as u64) << 16) | (addr as u64);//64 bit so we don't need to worry about overflow if increment_amount is large
    combined_addr -= decrement_amount as u64;//FIXME what about underflow?
    return (((combined_addr >> 16) & 0b111111) as u8, (combined_addr & 0xFFFF) as u16);
}
