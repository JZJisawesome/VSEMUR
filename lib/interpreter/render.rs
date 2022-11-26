/* render.rs
 * By: John Jekel
 *
 * TODO description
 *
*/

//TODO remove this once everything is implemented
#![allow(unused_variables)]
#![allow(dead_code)]
#[allow(unused_imports)]

/* Imports */

use crate::logging::log;

use crate::interpreter::common::ReadableMemory;
use crate::interpreter::common::WritableMemory;

use crate::interpreter::render_reciever::RenderReciever;

/* Constants */

//TODO

/* Macros */

//TODO (also pub(crate) use the_macro statements here too)

/* Static Variables */

//TODO

/* Types */

pub(super) struct RenderState {
    //TODO
}

/* Associated Functions and Methods */

impl RenderState {
    pub(super) fn new() -> RenderState {
        log!(2, "Initializing render state");

        //TODO implement
        return RenderState {};
        //unimplemented!();
    }

    pub(super) fn reset(self: &mut Self) {
        log!(2, "Resetting render state");
        //TODO zero out registers in mem
        //unimplemented!();//TODO
    }

    pub fn tick(self: &mut Self) {
        //unimplemented!();//TODO
        //TESTING
        //log!(1, "Printing render register contents");
        //for i in 0x002800..=0x0028FF {
        //    log!(2, "{:#08X}: {:#06X}", i, mem.read_addr(i));
        //}
    }

    pub(super) fn get_render_reciever(self: &mut Self) -> RenderReciever {
        todo!();
    }
}

impl ReadableMemory for RenderState {
    fn read_addr(self: &Self, addr: u32) -> u16 {
        log!(2, "Render Access");
        //todo!();
        return 0;//TODO implement sound (at least the registers so that the CPU gets the values it expects)
    }
}


impl WritableMemory for RenderState {
    fn write_addr(self: &mut Self, addr: u32, data: u16) {
        log!(2, "Render Access");
        //todo!();//TODO implement sound
    }
}

/* Functions */

//TODO
