/* state.rs
 * Copyright (C) 2022 John Jekel
 * See the LICENSE file at the root of the project for licensing info.
 *
 * Wrapper struct for memories, MMIO peripherals, interrupt sources, and CPU registers
 *
 * Implements many traits so that users of the struct can have restricted access to only the parts they need
 *
*/

//TODO remove this once everything is implemented
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(dead_code)]

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

/* Types */


pub(super) struct State {
    cpu_regs: cpu_regs::CPURegs,
    render: render::RenderState,
    sound: sound::SoundState,
    io: io::IOState,
    work_ram: Box<[u16]>,
    bios: bios::Bios,
    cartridge: cartridge::Cartridge,
    int_ctrl_reg: u16,//Used for interrupt handling
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

            #[cfg(not(feature = "nightly-features"))]
            work_ram: vec![0u16; PHYSICAL_MEM_SIZE_WORDS].into_boxed_slice(),//TODO avoid vector for speed//TODO avoid zero-initializing for speed
            #[cfg(feature = "nightly-features")]
            work_ram: box [0u16; PHYSICAL_MEM_SIZE_WORDS],

            bios: bios::Bios::new(),
            cartridge: cartridge::Cartridge::new(),
            int_ctrl_reg: 0,
        };
    }

    pub(super) fn reset(self: &mut Self) {
        log!(1, "Resetting VSmile state");

        self.reset_cpu();
        self.render.reset();
        self.sound.reset();
        self.io.reset();
        //TODO what to reset int_ctrl_reg to?
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
        return self.render.get_render_reciever();
    }

    pub(super) fn get_sound_reciever(self: &mut Self) -> SoundReciever {
        return self.sound.get_sound_reciever();
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

/* Tests */

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn sanity() {
        let mut state = init_new_random_state();
        assert!(!state.frame_ended());//We haven't executed any instructions, so it couldn't have possibly have ended already
        let render_reciever = state.get_render_reciever();
        let sound_reciever = state.get_sound_reciever();
        let input_sender = state.get_input_sender();
        for _ in 0..10000 {//Even though we aren't executing instructions, we should be able to survive some ticks without panicking
            state.tick();
        }
    }

    fn init_new_random_state() -> State {
        let mut state = State::new();
        state.load_bios_mem(&get_random_u16_slice(crate::interpreter::common::MAX_BIOS_SIZE_WORDS));
        state.load_rom_mem(&get_random_u16_slice(crate::interpreter::common::MAX_ROM_SIZE_WORDS));
        state.reset();
        return state;
    }

    fn get_random_u16_slice(size: usize) -> Box<[u16]> {
        let mut the_box = vec![0u16; size].into_boxed_slice();
        for i in 0..size {
            the_box[i] = i as u16;//TODO make actually random
        }
        return the_box;
    }
}

/* Benches */

#[cfg_attr(feature = "nightly-features", cfg(test))]
#[cfg(feature = "nightly-features")]
mod benches {
    extern crate test;
    use super::*;
    use test::Bencher;

    #[bench]
    fn create_new_state(b: &mut Bencher) {
        b.iter(|| -> State {
            return State::new();
        });
    }

    #[bench]
    fn tick(b: &mut Bencher) {
        let mut state = init_new_random_state();
        b.iter(|| {
            let mut state_reference = test::black_box(&mut state);
            state_reference.tick();
        });
    }

    fn init_new_random_state() -> State {
        let mut state = State::new();
        state.load_bios_mem(&get_random_u16_slice(crate::interpreter::common::MAX_BIOS_SIZE_WORDS));
        state.load_rom_mem(&get_random_u16_slice(crate::interpreter::common::MAX_ROM_SIZE_WORDS));
        state.reset();
        return state;
    }

    fn get_random_u16_slice(size: usize) -> Box<[u16]> {
        let mut the_box = vec![0u16; size].into_boxed_slice();
        for i in 0..size {
            the_box[i] = i as u16;//TODO make actually random
        }
        return the_box;
    }
}
