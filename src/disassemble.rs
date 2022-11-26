/* vsemur-disassemble
 * Copyright (C) 2022 John Jekel
 * See the LICENSE file at the root of the project for licensing info.
 *
 * Command-line frontend for exposed libvsemur disassembler facilities
 *
 * A great way to validate the correctness of VSEMUR's decode logic by comparing the output to MAME's unidasm
*/

//!Command-line frontend for exposed libvsemur disassembler facilities
//!
//!A great way to validate the correctness of VSEMUR's decode logic by comparing the output to MAME's unidasm

/* Imports */

use std::io::Write;
use std::io::Read;

/* Constants */

//TODO

/* Macros */

//TODO (also pub(crate) use the_macro statements here too)

/* Static Variables */

//TODO

/* Types */

//TODO

/* Associated Functions and Methods */

//TODO

/* Functions */

fn main() {
    //Print version info
    eprintln!("VSEMUR Disassembler");
    eprintln!("Powered by: {}\n", vsemur::about::version::pretty_string());

    //Handle command line arguments
    match std::env::args().len() {
        2 => {
            if std::env::args().nth(1).unwrap() == "--version" {
                eprintln!("{}", vsemur::about::LICENSE);
                return;
            } else {
                eprintln!("\x1b[31mError: Invalid argument\x1b[0m\n");
                return;
            }
        },
        3 => {},//Continue to the next part of main()
        _ => {
            eprintln!("\x1b[31mError: Expected 1 or 2 arguments (path to file to disassembly, path to output disassembly file; or --version)\x1b[0m\n");
            return;
        },
    }

    //Open files//TODO proper error handling
    let mut input_file = std::fs::OpenOptions::new().read(true).open(std::env::args().nth(1).unwrap()).unwrap();
    let output_file = std::fs::OpenOptions::new().write(true).create(true).truncate(true).open(std::env::args().nth(2).unwrap()).unwrap();
    let mut output_file_buffer = std::io::BufWriter::new(output_file);

    //Get the length of the input file
    let metadata = input_file.metadata().unwrap();//TODO proper error handling
    if (metadata.len() & 0b1) == 0b1 {//Ensure the file is a multiple of 2
        panic!();
    }

    //Determine the actual number of total addresses in the file (16-bit words)
    let total_addresses: usize = (metadata.len() / 2) as usize;

    //Output should look like MAME's unidasm so we can easily compare it and verify we are decoding instructions properly
    let addr_width: usize;
    match total_addresses {
        0x0..=0x10 => { addr_width = 1; },
        0x11..=0x100 => { addr_width = 2; },
        0x101..=0x1000 => { addr_width = 3; },
        0x1001..=0x10000 => { addr_width = 4; },
        0x10001..=0x100000 => { addr_width = 5; },
        0x100001..=0x1000000 => { addr_width = 6; },
        _ => { panic!(); },
    }

    eprint!("{}", addr_width);

    let mut byte_buffer: Box<[u8]> = vec![0u8; metadata.len() as usize].into_boxed_slice();//TODO avoid overhead of zeroing out contents, as well as overhead of needing to copy to buffer instead of reading to it directly
    input_file.read(&mut byte_buffer).unwrap();

    //Disassembly loop (Files are little-endian)
    let mut addr: usize = 0;
    while addr < total_addresses {
        //Get the first instruction wordgroup 1
        let wg1: u16 = ((byte_buffer[(addr * 2) + 1] as u16) << 8) | (byte_buffer[addr * 2] as u16);

        //Decode it
        let mut decoded_inst = vsemur::decode::DecodedInstruction::Invalid;
        vsemur::decode::decode_wg1(wg1, &mut decoded_inst);

        //Check if we need a second wordgroup to finish the decode process (and there is in fact another word group to get)
        if vsemur::decode::needs_decode_wg2(&decoded_inst) && (addr != (total_addresses - 1)) {
            //Get the second instruction wordgroup and finish the decoding process
            let wg2 = ((byte_buffer[(addr * 2) + 3] as u16) << 8) | (byte_buffer[(addr * 2) + 2] as u16);
            vsemur::decode::decode_wg2(&mut decoded_inst, wg2);

            //Write out to the log file:
            writeln!(&mut output_file_buffer, "{:0width$x}: {:04x} {:04x}  {}",
                addr,
                wg1,
                wg2,
                vsemur::decode::disassemble_mame_style(&decoded_inst, true, addr as u32),//TODO switch between this and other styles based on a command line param
                width = addr_width).unwrap();

            //Increment the address
            addr += 2;
        } else {
            //Write out to the log file:
            writeln!(&mut output_file_buffer, "{:0width$x}: {:04x}       {}",
                addr,
                wg1,
                vsemur::decode::disassemble_mame_style(&decoded_inst, true, addr as u32),
                width = addr_width).unwrap();

            //Increment the address
            addr += 1;
        }
    }
}
