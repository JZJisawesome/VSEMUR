/* NAME//TODO
 * By: John Jekel
 *
 * TODO description
 *
*/

/* Imports */

use crate::debug_panic;

use crate::logging::log;

use crate::interpreter::common::ReadableMemory;
use crate::interpreter::common::WritableMemory;

/* Constants */

const NUM_REGS: usize = 16;
const BASE_ADDR: u32 = 0x3D00;

mod offset {
    pub(super) const REG_IO_MODE: u32 = 0x0000;
    pub(super) const REG_IOA_DATA: u32 = 0x0001;
    pub(super) const REG_IOA_BUFFER: u32 = 0x0002;
    pub(super) const REG_IOA_DIR: u32 = 0x0003;
    pub(super) const REG_IOA_ATTRIB: u32 = 0x0004;
    pub(super) const REG_IOA_MASK: u32 = 0x0005;
    pub(super) const REG_IOB_DATA: u32 = 0x0006;
    pub(super) const REG_IOB_BUFFER: u32 = 0x0007;
    pub(super) const REG_IOB_DIR: u32 = 0x0008;
    pub(super) const REG_IOB_ATTRIB: u32 = 0x0009;
    pub(super) const REG_IOB_MASK: u32 = 0x000A;
    pub(super) const REG_IOC_DATA: u32 = 0x000B;
    pub(super) const REG_IOC_BUFFER: u32 = 0x000C;
    pub(super) const REG_IOC_DIR: u32 = 0x000D;
    pub(super) const REG_IOC_ATTRIB: u32 = 0x000E;
    pub(super) const REG_IOC_MASK: u32 = 0x000F;
}

/* Macros */

//TODO (also pub(crate) use the_macro statements here too)

/* Static Variables */

//TODO

/* Types */

pub(super) struct GPIO {
    registers: [u16; NUM_REGS]
}

/* Associated Functions and Methods */

impl GPIO {
    pub(super) fn new() -> GPIO {
        log!(3, "Initializing GPIO");

        return GPIO {
            registers: [0; NUM_REGS],//TODO avoid zero-init
        };
    }
}

impl ReadableMemory for GPIO {
    fn read_addr(self: &Self, addr: u32) -> u16 {
        return self.registers[(addr - BASE_ADDR) as usize];
        //TODO log the register we accessed
        //todo!();//TODO can we actually get away without implementing GPIO?
    }
}

impl WritableMemory for GPIO {
    fn write_addr(self: &mut Self, addr: u32, data: u16) {
        self.registers[(addr - BASE_ADDR) as usize] = data;
        //TODO log the register we accessed
        //todo!();//TODO can we actually get away without implementing GPIO?
    }
}

/* Functions */

//TODO
