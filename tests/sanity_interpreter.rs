/* sanity_interpreter.rs
 * Copyright (C) 2022 John Jekel
 * See the LICENSE file at the root of the project for licensing info.
 *
 * Sanity integration test for the interpreter
 *
*/

#[test]
fn sanity_interpreter() {
    //Initialize emulator
    let mut emulator = vsemur::interpreter::Emulator::new();
    emulator.load_bios_mem(&get_random_u16_slice(1 << 22));
    emulator.load_rom_mem(&get_random_u16_slice(1 << 22));
    emulator.reset();//Power-on reset AFTER loading bios and rom
    //TODO other setup (channels)

    assert!(!emulator.thread_running());
    eprintln!("starting");
    emulator.launch_emulation_thread();
    assert!(emulator.thread_running());
    eprintln!("started");
    std::thread::sleep(std::time::Duration::from_millis(1));//Long enough that we'll be past the exit check point and will do an entire frame
    eprintln!("stopping");
    emulator.stop_emulation_thread();
    assert!(!emulator.thread_running());
    eprintln!("stopped");

    //If we made it here, we did an entire frame without panicing!
}

fn get_random_u16_slice(size: usize) -> Box<[u16]> {
    let mut the_box = vec![0u16; size].into_boxed_slice();
    for i in 0..size {
        the_box[i] = i as u16;//TODO make actually random
    }
    return the_box;
}
