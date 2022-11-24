/* NAME//TODO
 * By: John Jekel
 *
 * TODO description
 *
*/

/* Imports */

use crate::debug_panic;

use crate::logging::log;

use crate::interpreter::common::Memory;

/* Constants */

const NUM_REGS: usize = 0x3;

const BASE_ADDR: u32 = 0x3D25;

mod offset {
    pub(super) const REG_ADC_CTRL: u32 = 0x0000;
    pub(super) const REG_ADC_PAD: u32 = 0x0001;
    pub(super) const REG_ADC_DATA: u32 = 0x0002;
}

/* Macros */

//TODO (also pub(crate) use the_macro statements here too)

/* Static Variables */

//TODO

/* Types */

pub(super) struct ADC {
    //registers: [u16; NUM_REGS]
    adc_ctrl: u16,
    adc_pad: u16,
    adc_data: u16,
}

/* Associated Functions and Methods */

impl ADC {
    pub(super) fn new() -> ADC {
        log!(3, "Initializing ADC");

        return ADC {
            adc_ctrl: 0,//TODO what should these default to?
            adc_pad: 0,//TODO what should these default to?
            adc_data: 0,//TODO what should these default to?
        };
    }
}

impl Memory for ADC {
    fn read_addr(self: &Self, addr: u32) -> u16 {
        match addr - BASE_ADDR {
            offset::REG_ADC_CTRL => { log!(4, "REG_ADC_CTRL"); return self.adc_ctrl; }//TODO can we actually get away without implementing the ADC?
            offset::REG_ADC_PAD => { log!(4, "REG_ADC_CTRL"); return self.adc_pad; }//TODO can we actually get away without implementing the ADC?
            offset::REG_ADC_DATA => { log!(4, "REG_ADC_CTRL"); return self.adc_data; }//TODO can we actually get away without implementing the ADC?
            _ => { return debug_panic!(0); },//Invalid address or access to unallocated address space
        }
    }

    fn write_addr(self: &mut Self, addr: u32, data: u16) {
        match addr - BASE_ADDR {
            offset::REG_ADC_CTRL => { log!(4, "REG_ADC_CTRL"); self.adc_ctrl = data; }//TODO can we actually get away without implementing the ADC?
            offset::REG_ADC_PAD => { log!(4, "REG_ADC_CTRL"); self.adc_pad = data; }//TODO can we actually get away without implementing the ADC?
            offset::REG_ADC_DATA => { log!(4, "REG_ADC_CTRL"); self.adc_data = data; }//TODO can we actually get away without implementing the ADC?
            _ => { debug_panic!(); },//Invalid address or access to unallocated address space
        }
    }
}

/* Functions */

//TODO
