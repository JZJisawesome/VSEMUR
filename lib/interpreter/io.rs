/* NAME//TODO
 * By: John Jekel
 *
 * NOTE: Since unlike unSP there is limited documentation avaliable for SPG240 peripherals, I may be leaning slighly more on MAME's code for this.
 *
*/

//TODO remove this once everything is implemented
#![allow(unused_variables)]
#![allow(dead_code)]
#[allow(unused_imports)]

/* Imports */

use crate::logging::log;

use super::memory::MemoryState;
use super::common::Memory;

/* Constants */

const BASE_ADDR: u32 = 0x3D00;

mod offset {

const REG_IO_MODE: u32 = 0x0000;
const REG_IOA_DATA: u32 = 0x0001;
const REG_IOA_BUFFER: u32 = 0x0002;
const REG_IOA_DIR: u32 = 0x0003;
const REG_IOA_ATTRIB: u32 = 0x0004;
const REG_IOA_MASK: u32 = 0x0005;
const REG_IOB_DATA: u32 = 0x0006;
const REG_IOB_BUFFER: u32 = 0x0007;
const REG_IOB_DIR: u32 = 0x0008;
const REG_IOB_ATTRIB: u32 = 0x0009;
const REG_IOB_MASK: u32 = 0x000A;
const REG_IOC_DATA: u32 = 0x000B;
const REG_IOC_BUFFER: u32 = 0x000C;
const REG_IOC_DIR: u32 = 0x000D;
const REG_IOC_ATTRIB: u32 = 0x000E;
const REG_IOC_MASK: u32 = 0x000F;

const REG_TIMEBASE_SETUP: u32 = 0x0010;
const REG_TIMEBASE_CLEAR: u32 = 0x0011;
const REG_TIMERA_DATA: u32 = 0x0012;
const REG_TIMERA_CTRL: u32 = 0x0013;
const REG_TIMERA_ON: u32 = 0x0014;
const REG_TIMERA_IRQCLR: u32 = 0x0015;
const REG_TIMERB_DATA: u32 = 0x0016;
const REG_TIMERB_CTRL: u32 = 0x0017;
const REG_TIMERB_ON: u32 = 0x0018;
const REG_TIMERB_IRQCLR: u32 = 0x0019;

const REG_VERT_LINE: u32 = 0x001C;

const REG_SYSTEM_CTRL: u32 = 0x0020;
const REG_INT_CTRL: u32 = 0x0021;
const REG_INT_CLEAR: u32 = 0x0022;
const REG_EXT_MEMORY_CTRL: u32 = 0x0023;
const REG_WATCHDOG_CLEAR: u32 = 0x0024;
const REG_ADC_CTRL: u32 = 0x0025;
const REG_ADC_PAD: u32 = 0x0026;
const REG_ADC_DATA: u32 = 0x0027;

const REG_SLEEP_MODE: u32 = 0x0028;
const REG_WAKEUP_SOURCE: u32 = 0x0029;
const REG_WAKEUP_TIME: u32 = 0x002A;

const REG_NTSC_PAL: u32 = 0x002B;

const REG_PRNG1: u32 = 0x002C;
const REG_PRNG2: u32 = 0x002D;

const REG_FIQ_SEL: u32 = 0x002E;
const REG_DATA_SEGMENT: u32 = 0x002F;

const REG_UART_CTRL: u32 = 0x0030;
const REG_UART_STATUS: u32 = 0x0031;
const REG_UART_RESET: u32 = 0x0032;
const REG_UART_BAUD1: u32 = 0x0033;
const REG_UART_BAUD2: u32 = 0x0034;
const REG_UART_TXBUF: u32 = 0x0035;
const REG_UART_RXBUF: u32 = 0x0036;
const REG_UART_RXFIFO: u32 = 0x0037;

const REG_SPI_CTRL: u32 = 0x0040;
const REG_SPI_TXSTATUS: u32 = 0x0041;
const REG_SPI_TXDATA: u32 = 0x0042;
const REG_SPI_RXSTATUS: u32 = 0x0043;
const REG_SPI_RXDATA: u32 = 0x0044;
const REG_SPI_MISC: u32 = 0x0045;

const REG_SIO_SETUP: u32 = 0x0050;
const REG_SIO_STATUS: u32 = 0x0051;
const REG_SIO_ADDRL: u32 = 0x0052;
const REG_SIO_ADDRH: u32 = 0x0053;
const REG_SIO_DATA: u32 = 0x0054;
const REG_SIO_AUTO_TX_NUM: u32 = 0x0055;

const REG_I2C_CMD: u32 = 0x0058;
const REG_I2C_STATUS: u32 = 0x0059;
const REG_I2C_ACCESS: u32 = 0x005A;
const REG_I2C_ADDR: u32 = 0x005B;
const REG_I2C_SUBADDR: u32 = 0x005C;
const REG_I2C_DATA_OUT: u32 = 0x005D;
const REG_I2C_DATA_IN: u32 = 0x005E;
const REG_I2C_MODE: u32 = 0x005F;

const REG_REGULATOR_CTRL: u32 = 0x60;
const REG_CLOCK_CTRL: u32 = 0x0061;
const REG_IO_DRIVE_CTRL: u32 = 0x0062;

}

/* Macros */

//TODO (also pub(crate) use the_macro statements here too)

/* Static Variables */

//TODO

/* Types */

pub(super) struct IOState {
    //TODO
}

/* Associated Functions and Methods */

impl IOState {
    pub(super) fn new() -> IOState {
        log!(1, "Initializing I/O state");

        //TODO implement
        return IOState {};
        //unimplemented!();
    }

    pub(super) fn reset(self: &mut Self, mem: &mut MemoryState) {
        log!(1, "Resetting I/O state");
        //TODO zero out registers in mem
        //unimplemented!();//TODO
    }

    pub fn tick(self: &mut Self, mem: &mut MemoryState) {
        log!(1, "I/O: TODO describe what we're doing");
        //unimplemented!();//TODO
        //TESTING
        //log!(1, "Printing render register contents");
        //for i in 0x002800..=0x0028FF {
        //    log!(2, "{:#08X}: {:#06X}", i, mem.read_addr(i));
        //}
    }
}

impl Memory for IOState {
    fn read_addr(self: &Self, addr: u32) -> u16 {
        todo!();
    }

    fn write_addr(self: &mut Self, addr: u32, data: u16) {
        todo!();
    }
}

/* Functions */

//TODO
