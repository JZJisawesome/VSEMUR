/* state.rs
 * By: John Jekel
 *
 * TODO description
 *
*/

/* Imports */

mod memory_map;
mod cpu_regs;
mod interrupts;

use cpu_regs::CPURegs;

use crate::debug_panic;

use crate::logging::log;

use super::common::CPU;
use crate::interpreter::common::InstructionMemory;
use crate::interpreter::common::ReadableMemory;
use crate::interpreter::common::WritableMemory;
use super::common::InterruptReadable;
use super::common::InterruptClearable;

use super::common::MEM_SIZE_WORDS;

use super::render_reciever::RenderReciever;
use super::sound_reciever::SoundReciever;
use super::input_sender::InputSender;

/* Constants */

//Page 47 is useful :)
const BREAK_INT_VECTOR_ADDR: usize = 0xFFF5;
const FIQ_INT_VECTOR_ADDR: usize = 0xFFF6;
const RESET_INT_VECTOR_ADDR: usize = 0xFFF7;
const IRQ_INT_VECTOR_ADDR: [usize;8] = [0xFFF8, 0xFFF9, 0xFFFA, 0xFFFB, 0xFFFC, 0xFFFD, 0xFFFE, 0xFFFF];//0 thru 7

/* Macros */

//TODO (also pub(crate) use the_macro statements here too)

/* Static Variables */

//TODO

/* Types */

#[derive(Default)]
pub(super) struct State {
    cpu_regs: CPURegs,
}

/* Associated Functions and Methods */

impl State {
    pub(super) fn new() -> State {
        log!(1, "Initializing VSmile state");
        return State::default();
    }

    pub(super) fn reset(self: &mut Self) {
        log!(1, "Resetting state");

        self.reset_cpu();
        //TODO others
        //self.render.reset();
        //self.sound.reset();
        //self.io.reset();
    }

    fn reset_cpu(self: &mut Self) {
        log!(1, "Resetting CPU");

        log!(2, "Initialize FR to 0bx_0_0_0_0_0000_0_0_0_1000");
        self.cpu_regs.fr = 0b0_0_0_0_0_0000_0_0_0_1000;

        log!(2, "Set initial CS page and PC");
        debug_assert!(RESET_INT_VECTOR_ADDR < MEM_SIZE_WORDS);
        log!(3, "Read reset vector at address {:#04X}_{:04X}", RESET_INT_VECTOR_ADDR >> 16, RESET_INT_VECTOR_ADDR & 0xFFFF);
        self.set_cs(0x00);
        self.cpu_regs.pc = self.read_addr(RESET_INT_VECTOR_ADDR as u32);
        log!(3, "Initial CS page, PC is {:#04X}_{:04X}", self.get_cs(), self.cpu_regs.pc);
    }

    pub(super) fn tick(self: &mut Self) {
        //log!(1, "Peripherals: Tick begins");
        //todo!();//TODO
        //log!(1, "Peripherals: Tick ends");
    }

    pub(super) fn frame_ended(self: &mut Self) -> bool {
        return false;//TODO
    }

    pub(super) fn get_render_reciever(self: &mut Self) -> RenderReciever {
        todo!();
    }

    pub(super) fn get_sound_reciever(self: &mut Self) -> SoundReciever {
        todo!();
    }

    pub(super) fn get_input_sender(self: &mut Self) -> InputSender {
        todo!();
    }

    pub(super) fn load_bios_file(self: &mut Self, path: &str) -> Result<(), ()> {
        todo!();
    }

    pub(super) fn load_bios_mem(self: &mut Self, bios_mem: &[u16]) -> Result<(), ()> {
        todo!();
    }

    pub(super) fn load_rom_file(self: &mut Self, path: &str) -> Result<(), ()> {
        todo!();
    }

    pub(super) fn load_rom_mem(self: &mut Self, rom_mem: &[u16]) -> Result<(), ()> {
        todo!();
    }

}

/* Functions */

//TODO
