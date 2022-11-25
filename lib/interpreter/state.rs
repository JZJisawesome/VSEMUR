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

use super::render;
use super::sound;
use super::io;
use super::bios;
use super::cartridge;

use crate::debug_panic;

use crate::logging::log;

use crate::interpreter::common::PHYSICAL_MEM_SIZE_WORDS;

use super::render_reciever::RenderReciever;
use super::sound_reciever::SoundReciever;
use super::input_sender::InputSender;

/* Constants */

//TODO

/* Macros */

//TODO (also pub(crate) use the_macro statements here too)

/* Static Variables */

//TODO

/* Types */


pub(super) struct State {
    cpu_regs: cpu_regs::CPURegs,
    render: render::RenderState,
    sound: sound::SoundState,
    io: io::IOState,
    work_ram: Box<[u16]>,
    bios: bios::Bios,
    cartridge: cartridge::Cartridge,
    //TODO "peripherals" won't go into the state directory, but will rather be "peers" of state and unsp in the interpreter directory
}

/* Associated Functions and Methods */

impl State {
    pub(super) fn new() -> State {
        log!(1, "Initializing VSmile state");
        return State {
            cpu_regs: cpu_regs::CPURegs::default(),
            render: render::RenderState::new(),
            sound: sound::SoundState::new(),
            io: io::IOState::new(),
            work_ram: vec![0u16; PHYSICAL_MEM_SIZE_WORDS].into_boxed_slice(),//TODO avoid vector for speed//TODO avoid zero-initializing for speed
            bios: bios::Bios::new(),
            cartridge: cartridge::Cartridge::new(),
        };
    }

    pub(super) fn reset(self: &mut Self) {
        log!(1, "Resetting VSmile state");

        self.reset_cpu();
        self.render.reset();
        self.sound.reset();
        self.io.reset();
    }

    pub(super) fn tick(self: &mut Self) {
        log!(2, "Peripherals: Tick begins");
        //todo!();//TODO
        log!(2, "Peripherals: Tick ends");
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
        return self.bios.load_file(path);
    }

    pub(super) fn load_bios_mem(self: &mut Self, bios_mem: &[u16]) -> Result<(), ()> {
        return self.bios.load_mem(bios_mem);
    }

    pub(super) fn load_rom_file(self: &mut Self, path: &str) -> Result<(), ()> {
        return self.cartridge.load_file(path);
    }

    pub(super) fn load_rom_mem(self: &mut Self, rom_mem: &[u16]) -> Result<(), ()> {
        return self.cartridge.load_mem(rom_mem);
    }

}

/* Functions */

//TODO
