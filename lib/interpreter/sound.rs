/* sound.rs
 * Copyright (C) 2022 John Jekel
 * See the LICENSE file at the root of the project for licensing info.
 *
 * TODO description
 *
*/

//TODO remove this once everything is implemented
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(dead_code)]

/* Imports */

use crate::logging::log;

use crate::interpreter::common::ReadableMemory;
use crate::interpreter::common::WritableMemory;

use crate::interpreter::sound_reciever::SoundReciever;

/* Constants */

//TODO

/* Macros */

//TODO (also pub(crate) use the_macro statements here too)

/* Static Variables */

//TODO

/* Types */

pub(super) struct SoundState {
    //TODO
}

/* Associated Functions and Methods */

impl SoundState {
    pub(super) fn new() -> SoundState {
        log!(2, "Initializing sound state");

        //TODO implement
        return SoundState {};
        //unimplemented!();
    }

    pub(super) fn reset(self: &mut Self) {
        log!(2, "Resetting sound state");
        //unimplemented!();//TODO
    }

    pub fn tick(self: &mut Self) {
        //unimplemented!();//TODO
    }

    pub(super) fn get_sound_reciever(self: &mut Self) -> SoundReciever {
        todo!();
    }
}

impl ReadableMemory for SoundState {
    fn read_addr(self: &Self, addr: u32) -> u16 {
        log!(2, "Sound Access");
        //todo!();
        return 0;//TODO implement sound (at least the registers so that the CPU gets the values it expects)
    }
}


impl WritableMemory for SoundState {
    fn write_addr(self: &mut Self, addr: u32, data: u16) {
        log!(2, "Sound Access");
        //todo!();//TODO implement sound
    }
}

/* Functions */

//TODO
