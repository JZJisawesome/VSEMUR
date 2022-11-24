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

use crate::interpreter::common::Memory;

/* Constants */

const NUM_REGS: usize = 0x100;

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

    pub(super) const REG_TIMEBASE_SETUP: u32 = 0x0010;
    pub(super) const REG_TIMEBASE_CLEAR: u32 = 0x0011;
    pub(super) const REG_TIMERA_DATA: u32 = 0x0012;
    pub(super) const REG_TIMERA_CTRL: u32 = 0x0013;
    pub(super) const REG_TIMERA_ON: u32 = 0x0014;
    pub(super) const REG_TIMERA_IRQCLR: u32 = 0x0015;
    pub(super) const REG_TIMERB_DATA: u32 = 0x0016;
    pub(super) const REG_TIMERB_CTRL: u32 = 0x0017;
    pub(super) const REG_TIMERB_ON: u32 = 0x0018;
    pub(super) const REG_TIMERB_IRQCLR: u32 = 0x0019;

    pub(super) const REG_VERT_LINE: u32 = 0x001C;

    pub(super) const REG_SYSTEM_CTRL: u32 = 0x0020;
    pub(super) const REG_INT_CTRL: u32 = 0x0021;
    pub(super) const REG_INT_CLEAR: u32 = 0x0022;
    pub(super) const REG_EXT_MEMORY_CTRL: u32 = 0x0023;//NOTE: This isn't handled in IO, but rather in the logic for the cartridge
    pub(super) const REG_WATCHDOG_CLEAR: u32 = 0x0024;
    pub(super) const REG_ADC_CTRL: u32 = 0x0025;
    pub(super) const REG_ADC_PAD: u32 = 0x0026;
    pub(super) const REG_ADC_DATA: u32 = 0x0027;

    pub(super) const REG_SLEEP_MODE: u32 = 0x0028;
    pub(super) const REG_WAKEUP_SOURCE: u32 = 0x0029;
    pub(super) const REG_WAKEUP_TIME: u32 = 0x002A;

    pub(super) const REG_NTSC_PAL: u32 = 0x002B;

    pub(super) const REG_PRNG1: u32 = 0x002C;
    pub(super) const REG_PRNG2: u32 = 0x002D;

    pub(super) const REG_FIQ_SEL: u32 = 0x002E;
    pub(super) const REG_DATA_SEGMENT: u32 = 0x002F;

    pub(super) const REG_UART_CTRL: u32 = 0x0030;
    pub(super) const REG_UART_STATUS: u32 = 0x0031;
    pub(super) const REG_UART_RESET: u32 = 0x0032;
    pub(super) const REG_UART_BAUD1: u32 = 0x0033;
    pub(super) const REG_UART_BAUD2: u32 = 0x0034;
    pub(super) const REG_UART_TXBUF: u32 = 0x0035;
    pub(super) const REG_UART_RXBUF: u32 = 0x0036;
    pub(super) const REG_UART_RXFIFO: u32 = 0x0037;

    pub(super) const REG_SPI_CTRL: u32 = 0x0040;
    pub(super) const REG_SPI_TXSTATUS: u32 = 0x0041;
    pub(super) const REG_SPI_TXDATA: u32 = 0x0042;
    pub(super) const REG_SPI_RXSTATUS: u32 = 0x0043;
    pub(super) const REG_SPI_RXDATA: u32 = 0x0044;
    pub(super) const REG_SPI_MISC: u32 = 0x0045;

    pub(super) const REG_SIO_SETUP: u32 = 0x0050;
    pub(super) const REG_SIO_STATUS: u32 = 0x0051;
    pub(super) const REG_SIO_ADDRL: u32 = 0x0052;
    pub(super) const REG_SIO_ADDRH: u32 = 0x0053;
    pub(super) const REG_SIO_DATA: u32 = 0x0054;
    pub(super) const REG_SIO_AUTO_TX_NUM: u32 = 0x0055;

    pub(super) const REG_I2C_CMD: u32 = 0x0058;
    pub(super) const REG_I2C_STATUS: u32 = 0x0059;
    pub(super) const REG_I2C_ACCESS: u32 = 0x005A;
    pub(super) const REG_I2C_ADDR: u32 = 0x005B;
    pub(super) const REG_I2C_SUBADDR: u32 = 0x005C;
    pub(super) const REG_I2C_DATA_OUT: u32 = 0x005D;
    pub(super) const REG_I2C_DATA_IN: u32 = 0x005E;
    pub(super) const REG_I2C_MODE: u32 = 0x005F;

    pub(super) const REG_REGULATOR_CTRL: u32 = 0x60;
    pub(super) const REG_CLOCK_CTRL: u32 = 0x0061;
    pub(super) const REG_IO_DRIVE_CTRL: u32 = 0x0062;
}

/* Macros */

//TODO (also pub(crate) use the_macro statements here too)

/* Static Variables */

//TODO

/* Types */

pub(super) struct IOState {
    registers: [u16; NUM_REGS],
}

/* Associated Functions and Methods */

impl IOState {
    pub(super) fn new() -> IOState {
        log!(2, "Initializing I/O state");

        //TODO implement
        return IOState {
            registers: [0; NUM_REGS],//TODO avoid zero-init

        };
        //unimplemented!();
    }

    pub(super) fn reset(self: &mut Self) {
        log!(2, "Resetting I/O state");
        //TODO reset self.registers to the proper initial states
        //self.registers[offset::REG_EXT_MEMORY_CTRL as usize] = 0x0028;//NOTE: This isn't handled in IO, but rather in the logic for the cartridge
        //unimplemented!();//TODO
    }

    pub fn tick(self: &mut Self) {
        log!(2, "I/O: TODO describe what we're doing");
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
        log!(2, "I/O Access");
        match addr - BASE_ADDR {
            //offset::REG_EXT_MEMORY_CTRL => { log!(3, "REG_EXT_MEMORY_CTRL read"); return self.registers[offset::REG_EXT_MEMORY_CTRL as usize]; },//NOTE: This isn't handled in IO, but rather in the logic for the cartridge
            _ => { todo!(); }//TODO implement
        }
    }

    fn write_addr(self: &mut Self, addr: u32, data: u16) {
        log!(2, "I/O Access");
        match addr - BASE_ADDR {
            offset::REG_WATCHDOG_CLEAR => { log!(3, "REG_WATCHDOG_CLEAR written with {}correct value", if data == 0x55AA { "" } else { "in" }); },//We don't actually implement the WDT
            _ => { todo!(); }//TODO implement
        }
    }
}

/* Functions */

//TODO
