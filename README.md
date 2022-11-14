# VSEMUR

VSmile EMUlator in Rust

To make things a bit more interesting, I decided to switch my VSEMU project (http://git.jekel.ca/JZJ/VSEMU) to being written in Rust. Not only is this a good excuse to learn Rust, which I would have had to at some point anyways, but also makes the project a bit more novel in comparison to the emulator that already exists in MAME!

Note: When developing this, I try to gleam as much of my understanding of the behaviour of the system/CPU architecture as possible for publically available documentation. However, at times, it becomes
necessary to view MAME's implementation when the documents available are unclear. I try to avoid this as much as possible, and rarely if ever actually copy code verbatim, but to be safe I consider this project to be a partial derivative work of MAME, and am (to the best of my knowledge) following its GPLv2 license properly.

Note: the unSP Programmer's Guide is a very useful PDF that comes with Sunplus/Generalplus's IDE. It was really helpful when working on this.

## unSP 1.2 Instruction Decoding

This was one of the hardest challenges of this project. At first, I was just going about it in a similar way to MAME, however I felt it to be confusing and it tripped me up a couple of times.
This therefore are the steps I ended up taking to decode instructions (see the spreadsheet in the root of the project too). It borrows some behaviour from the way MAME does it (particularly when the ISA is unclear as to which instruction should take priority), but has quite a few differences (for example, I do decoding the instruction type seperately from decoding other fields and actual execution).
Note: the upper nibble refers to bits [15:12] of the instruction word and the secondary group is bits [8;6], and Rd is bits [11:9]

1. Check if the instruction is 0xFFFF or 0x0000, in which case it's invalid.
2. Check if the instruction has an upper nibble of 0b1111. If so, do the following substeps, else proceed to 3
    Case secondary group is 0b000:
        - If Rd is 0b111 (aka the PC), the instruction is DSI6 (Note: I was worried this wasn't correct, but I check with MAME and it is). Else keep going
        - If bits [5:4] are 0b00, it is MUL; if they're 0b10, it is DS Access; if they're 0b11, they're FR access; else the instruction is invalid.
    Case secondary group is 0b001:
        - If bit 9 is 0, it is CALL, else it is invalid.
    Case secondary group is 0b010:
        - If Rd is 0b111 (aka the PC), the instruction is JMPF (Note: I was worried this wasn't correct, but I check with MAME and it is). Else it is MULS
    Case secondary group is 0b011:
        - If Rd is 0b111 (aka the PC), the instruction is JMPR (Note: I was worried this wasn't correct, but I check with MAME and it is). Else it is MULS
    Case secondary group is 0b100:
        - It is MUL
    Case secondary group is 0b101:
        - It can be decoded trivially using the lower bits of the instruction (see the unSP documentation/ISA summary)
    Case secondary group is 0b110 or 0b111:
        - It is MULS
3. Check if the instruction has an upper nibble of 0b1110. If so, do the following substeps, else proceed to 4
    3a. If Rd is 0b111 AND the secondary group is either 0b000 or 0b001, it is Branch. Else keep going
    3b. Do the following substeps
        Case secondary group is 0b000:
            - If bit 3 is set, it is MUL, else it is Register BITOP (Rs)
        Case secondary group is 0b001:
            - It is Register BITOP (offset)
        Case secondary group is 0b010:
            - It is MULS
        Case secondary group is 0b011:
            - It is invalid
        Case secondary group is 0b100 or 0b101:
            - If bit 3 is set, it is 16 bits Shift, else it is Memory BITOP (Rs)
        Case secondary group is 0b110:
            - It is Memory BITOP (Offset). Although this conflicts with MULS and I had no idea what to do, in MAME's unsp emulator Memory BITOP (Offset) takes priority in this case.
        Case secondary group is 0b111:
            - It is Memory BITOP (Offset). Although this conflicts with MULS and I had no idea what to do, in MAME's unsp emulator Memory BITOP (Offset) takes priority in this case.
4. Check if the instruction has an upper nibble of 0b0101 or 0b0111. If it is, perform the substeps, else goto 5
    - Verify that Rd is 0b111 and the secondary group is 0b000 or 0b001. If so, it is a Branch, otherwise it's invalid.
5. For all other upper nibbles, do the following substeps:
    3a. If Rd is 0b111 AND the secondary group is either 0b000 or 0b001, it is Branch. Else keep going
    3b. It is one of the alu operation instructions. Look at the secondary group to decode it first, then look at the lower bits to figure out the instruction.
        This is mostly straightforward, except for IMM16 and Direct16, which are easy to tell apart on their own, but are hard when also considering Register (conflict with bits 5:3).
        The ISA docs are, again, not clear, but MAME does this:
        - IMM16 and Direct16 have "priority": so if bits 5:3 are 001 it is IMM16, and if the bits are 010 or 011 it is Direct16
        - Otherwise it is Register
