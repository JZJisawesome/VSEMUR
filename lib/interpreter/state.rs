/* NAME//TODO
 * By: John Jekel
 *
 * TODO description
 *
*/

/* Imports */

use crate::debug_panic;
use super::*;
use super::common::Memory;

/* Constants */

//TODO deal with banking

//All inclusive
const WORK_RAM_BEGIN_ADDR: u32 = 0x000000;
const WORK_RAM_END_ADDR: u32 = 0x0027FF;
const RENDER_BEGIN_ADDR: u32 = 0x002800;
const RENDER_END_ADDR: u32 = 0x002FFF;
const SOUND_BEGIN_ADDR: u32 = 0x003000;
const SOUND_END_ADDR: u32 = 0x0037FF;
const IO_BEGIN_ADDR: u32 = 0x003D00;
const IO_END_ADDR: u32 = 0x003DFF;
const DMA_BEGIN_ADDR: u32 = 0x003E00;
const DMA_END_ADDR: u32 = 0x003E03;
const BIOS_BEGIN_ADDR: u32 = 0x003E04;//TODO figure out what this is
const BIOS_END_ADDR: u32 = 0x0FFFFF;//TODO figure out what this is
const ROM_BEGIN_ADDR: u32 = 0x100000;//TODO figure out what this is
const ROM_END_ADDR: u32 = 0x3FFFFF;//TODO figure out what this is

/* Macros */

//TODO (also pub(crate) use the_macro statements here too)

/* Static Variables */

//TODO

/* Types */

///State for the VSEMUR interpreter
///
///Holds all information needed to store the state of an emulated VSmile system.
///
///Instanciate with [`State::new()`].
pub struct State {
    reset_needed: bool,

    cpu: cpu::CPUState,
    render: render::RenderState,
    sound: sound::SoundState,
    io: io::IOState,
    mem: memory::MemoryState,
}

/* Associated Functions and Methods */

//Public methods
impl State {
    ///Instanciates a new [`State`].
    ///
    ///You probably want to load a rom and bios after this; see [`State::load_bios_file()`], [`State::load_bios_mem()`], [`State::load_rom_file()`], and [`State::load_rom_mem()`].
    pub fn new() -> State {
        log_reset_file!();
        log_reset_ticks!();

        log_ansi!(0, "\x1b[1;97m", "Initializing VSEMUR State");

        let new_state = State {
            reset_needed: true,

            cpu: cpu::CPUState::new(),
            render: render::RenderState::new(),
            sound: sound::SoundState::new(),
            io: io::IOState::new(),
            mem: memory::MemoryState::new(),
        };

        log!(0, "Initialization complete");
        return new_state
    }

    ///Resets the emulated system.
    ///
    ///Requires that a rom and bios have already been loaded beforehand; see [`State::load_bios_file()`], [`State::load_bios_mem()`], [`State::load_rom_file()`], and [`State::load_rom_mem()`].
    ///
    ///Returns [`ReturnCode::ResetFail`] if a BIOS or ROM wasn't loaded beforehand; otherwise returns [`ReturnCode::ResetOk`].
    pub fn reset(self: &mut Self) -> ReturnCode {
        log_reset_ticks!();
        log_ansi!(0, "\x1b[1;97m", "Resetting emulated system");

        //Memory must be reset first since other parts may depend on values in it at reset
        let mem_result: bool = self.mem.reset();
        if cfg!(debug_assertions) {
            if !mem_result {//BIOS or ROM wasn't loaded
                return ReturnCode::ResetFail;
            }
        }

        self.cpu.reset(self);
        self.render.reset(&mut self.mem);
        self.sound.reset();
        self.io.reset(&mut self.mem);

        log!(0, "Reset complete");
        self.reset_needed = false;
        return ReturnCode::ResetOk;
    }

    /*pub fn cache(self: &mut Self) {
        self.cpu.cache(&mut self.mem);
    }*/

    ///Performs one "tick" of the emulated system, equivalent to one clock cycle.
    ///
    ///This function should be called approximately 27 million times per second (27 MHz)
    ///
    ///Before this is called, [`State::reset()`] should already have been called at least once.
    ///
    ///Returns [`ReturnCode::TickFail`] if the proper prerequisites have not been met. Otherwise normally returns [`ReturnCode::TickOk`], unless a new frame is ready to be shown to the user, in which case it returns [`ReturnCode::TickOkNewFrameAvailable`].
    pub fn tick(self: &mut Self) -> ReturnCode {
        if cfg!(debug_assertions) {
            if self.reset_needed {
                return ReturnCode::TickFail;
            }
        }

        //Increment the number of ticks for debugging
        log_increment_ticks!();
        log_ansi!(0, "\x1b[1;97m", "Tick begins");

        //Tick sub-states
        self.cpu.tick(self);
        self.render.tick(&mut self.mem);
        self.sound.tick();
        self.io.tick(&mut self.mem);

        log!(0, "Tick ends");
        return ReturnCode::TickOk;
    }

    /*pub fn tick_cached(self: &mut Self) -> ReturnCode {
        if cfg!(debug_assertions) {
            if self.reset_needed {
                return ReturnCode::TickFail;
            }
        }

        //Increment the number of ticks for debugging
        log_increment_ticks!();
        log_ansi!(0, "\x1b[1;97m", "Tick begins");

        //Tick sub-states
        self.cpu.tick_cached(&mut self.mem);
        self.render.tick(&mut self.mem);
        self.sound.tick();

        log!(0, "Tick ends");
        return ReturnCode::TickOk;
    }
    */

    ///Loads a VSmile BIOS file from disk at the path specified.
    ///
    ///After this function is called, [`State::reset()`] must be called before [`State::tick()`] is called again.
    ///
    ///Returns [`ReturnCode::LoadOk`] if the load was sucessful, [`ReturnCode::LoadFailOpen`] if there was a filesystem issue, [`ReturnCode::LoadFailSize`] if the file was an invalid size.
    pub fn load_bios_file(self: &mut Self, path: &str) -> ReturnCode {
        self.reset_needed = true;
        return self.mem.load_bios_file(path);
    }

    ///Loads a VSmile BIOS from the memory contained within the given slice.
    ///
    ///After this function is called, [`State::reset()`] must be called before [`State::tick()`] is called again.
    ///
    ///Returns [`ReturnCode::LoadOk`] if the load was sucessful, or [`ReturnCode::LoadFailSize`] if the slice was an invalid size.
    pub fn load_bios_mem(self: &mut Self, bios_mem: &[u16]) -> ReturnCode {
        self.reset_needed = true;
        return self.mem.load_bios_mem(bios_mem);
    }

    ///Loads a VSmile rom file from disk at the path specified.
    ///
    ///After this function is called, [`State::reset()`] must be called before [`State::tick()`] is called again.
    ///
    ///Returns [`ReturnCode::LoadOk`] if the load was sucessful, [`ReturnCode::LoadFailOpen`] if there was a filesystem issue, [`ReturnCode::LoadFailSize`] if the file was an invalid size.
    pub fn load_rom_file(self: &mut Self, path: &str) -> ReturnCode {
        self.reset_needed = true;
        return self.mem.load_rom_file(path);
    }

    ///Loads a VSmile rom from the memory contained within the given slice.
    ///
    ///After this function is called, [`State::reset()`] must be called before [`State::tick()`] is called again.
    ///
    ///Returns [`ReturnCode::LoadOk`] if the load was sucessful, or [`ReturnCode::LoadFailSize`] if the slice was an invalid size.
    pub fn load_rom_mem(self: &mut Self, rom_mem: &[u16]) -> ReturnCode {
        self.reset_needed = true;
        return self.mem.load_rom_mem(rom_mem);
    }

    //TODO functions to read frambuffer so the user can display it as they wish
    //Perhaps provide some sort of thread-safe function so we can render the screen asynchrenously?
    //Don't actually do rendering in render::tick(), just save the data we need to render later, and don't actually render until the user
    //calls our function to do so (which could be asynchrenous)
    //Idea:
    //State::latch_render() copies the values/data internally needed to render later (meaning we can tick right away afterwards since even if we affect the render settings we have a copy)
    // Or perhaps this returns some sort of FutureFramebuffer object that we can call a render function on, completely seperate from the main State?
    //State::render() returns the rendered framebuffer (the user can call this in a seperate thread)
}

impl Memory for State {
    /* Memory access functions */

    //Instead of accessing other children directly for memory accesses, modules should use these functions as they correctly map the address space to the appropriate location
    /*pub(super) fn fetch_addr(self: &Self, addr: u32) -> u16 {//Faster than read_addr, but it only can access the ROM/BIOS memory locations
        debug_assert!((addr as usize) <= MEM_SIZE_WORDS);
        debug_assert!(addr >= 0x008000);
        //todo!();
        //TODO for now we only read from memory
        return self.mem.read_addr(addr);
    }
    */

    fn read_addr(self: &Self, addr: u32) -> u16 {
        debug_assert!((addr as usize) <= MEM_SIZE_WORDS);

        if (addr >= 0x2800) && (addr <= 0x7FFF) {//TESTING
            log_ansi!(0, "\x1b[31m", "Read from location outside of memory or bios/rom: {:#06X}", addr);
        }
        //TODO for now we only read from memory
        return self.mem.read_addr(addr);
        /*match addr {
            WORK_RAM_BEGIN_ADDR..=WORK_RAM_END_ADDR => { todo!(); },
            RENDER_BEGIN_ADDR..=RENDER_END_ADDR => { todo!(); },
            SOUND_BEGIN_ADDR..=SOUND_END_ADDR => { todo!(); },
            IO_BEGIN_ADDR..=IO_END_ADDR => { todo!(); },
            DMA_BEGIN_ADDR..=DMA_END_ADDR => { todo!(); },
            BIOS_BEGIN_ADDR..=BIOS_END_ADDR => { todo!(); },
            ROM_BEGIN_ADDR..=ROM_END_ADDR => { todo!(); },
            _ => { return debug_panic!(0); },//Invalid address or access to unallocated address space
        }
        */
    }

    fn write_addr(self: &mut Self, addr: u32, data: u16) {
        debug_assert!((addr as usize) <= MEM_SIZE_WORDS);

                if addr >= 0x2800 {//TESTING
            log_ansi!(0, "\x1b[31m", "Write to location outside of memory: {:#06X}", addr);
        }

        //TODO for now we only write to memory
        return self.mem.write_addr(data, addr);

        /*match addr {
            WORK_RAM_BEGIN_ADDR..=WORK_RAM_END_ADDR => { todo!(); },
            RENDER_BEGIN_ADDR..=RENDER_END_ADDR => { todo!(); },
            SOUND_BEGIN_ADDR..=SOUND_END_ADDR => { todo!(); },
            IO_BEGIN_ADDR..=IO_END_ADDR => { todo!(); },
            DMA_BEGIN_ADDR..=DMA_END_ADDR => { todo!(); },
            BIOS_BEGIN_ADDR..=BIOS_END_ADDR => { todo!(); },
            ROM_BEGIN_ADDR..=ROM_END_ADDR => { todo!(); },
            _ => { return debug_panic!(0); },//Invalid address or access to unallocated address space
        }
        */
    }

    /*pub(super) fn fetch_page_addr(self: &Self, page: u8, addr: u16) -> u16 {
        return self.fetch_addr(((page as u32) << 16) | (addr as u32));
    }*/
}

/* Functions */
