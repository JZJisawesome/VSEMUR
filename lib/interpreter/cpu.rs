/* NAME//TODO
 * By: John Jekel
 *
 * TODO description
 *
*/

/* Imports */

mod execute;

use crate::logging::log;
use super::memory::MemoryState;
use super::MEM_SIZE_WORDS;

/* Constants */

const INT_VECTOR_BASE_ADDR: usize = 0xFFF5;//Page 47 is useful :)
const RESET_VECTOR_ADDR: usize = 0xFFF7;//Page 47 is useful :)

/* Macros */

//TODO (also pub(crate) use the_macro statements here too)

/* Static Variables */

//TODO

/* Types */

pub(super) struct CPUState {
    regs: Registers,
    irq_enabled: bool,
    fiq_enabled: bool,
}

struct Registers {
    sp: u16,
    r: [u16;4],
    bp: u16,
    sr: SR,
    pc: u16,
}

struct SR {//16 bits in all
    //6 bits each
    ds: u8,
    cs: u8,

    //1 bit each
    n: bool,
    z: bool,
    s: bool,
    c: bool,
}

struct Inst {
    wg: [u16; 2],
}

/* Associated Functions and Methods */

impl CPUState {
    pub(super) fn new() -> CPUState {
        log!(0, 1, "Initializizing CPU State");
        return CPUState {
            regs: Registers {
                sp: 0,
                r: [0, 0, 0, 0],
                bp: 0,
                sr: SR {
                    ds: 0,
                    cs: 0,

                    n: false,
                    z: false,
                    s: false,
                    c: false
                },
                pc: 0,
            },
            irq_enabled: false,
            fiq_enabled: false,
        };
    }

    pub(super) fn reset(self: &mut Self, mem: &MemoryState) {
        log!(0, 1, "Resetting CPU");

        log!(0, 2, "Zero out all registers to begin with");
        self.regs = Registers {
            sp: 0,
            r: [0, 0, 0, 0],
            bp: 0,
            sr: SR {
                ds: 0,
                cs: 0,

                n: false,
                z: false,
                s: false,
                c: false
            },
            pc: 0,
        };
        self.irq_enabled = false;
        self.fiq_enabled = false;

        log!(0, 2, "Set initial CS page and PC");
        debug_assert!(RESET_VECTOR_ADDR < MEM_SIZE_WORDS);
        log!(0, 3, "Read reset vector at address {:#04X}_{:04X}", RESET_VECTOR_ADDR >> 16, RESET_VECTOR_ADDR & 0xFFFF);
        self.regs.pc = mem.read_addr(RESET_VECTOR_ADDR);
        log!(0, 3, "Initial CS page, PC is {:#04X}_{:04X}", self.regs.sr.cs, self.regs.pc);

        //TODO do we need to initialize the cs or ds?
    }

    pub fn tick(self: &mut Self, t: u128, mem: &mut MemoryState) {
        debug_assert!(mem.ready());

        //Fetch instruction from memory
        debug_assert!(self.regs.sr.cs < 0b111111);
        log!(t, 1, "Fetch started from CS page, PC address: {:#04X}_{:04X}", self.regs.sr.cs, self.regs.pc);
        let inst_word: u16 = mem.read_page_addr(self.regs.sr.cs, self.regs.pc);
        log!(t, 2, "Instruction Word: {:#06X} | {:#018b}", inst_word, inst_word);

        //Execute it
        execute::execute(t, self, mem, inst_word);

        //TODO handle interrupts, etc
    }
}

/* Functions */
