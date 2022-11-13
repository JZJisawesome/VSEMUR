/* NAME//TODO
 * By: John Jekel
 *
 * TODO description
 *
*/

//DecodedInstructionType's case closely matches the ISA documentation
#[allow(non_camel_case_types)]

/* Imports */

//TODO (include "use" and "mod" here)

/* Constants */

//TODO

/* Macros */

//TODO (also pub(crate) use the_macro statements here too)

/* Static Variables */

//TODO

/* Types */

pub(super) enum DecodedInstructionType {
    DSI6,
    CALL,
    JMPF,
    JMPR,
    FIR_MOV,
    Fraction,
    INT_SET,
    IRQ,
    SECBANK,
    FIQ,
    IRQ_NEST_MODE,
    BREAK,
    CALLR,
    DIVS,
    DIVQ,
    EXP,
    NOP,
    DS_Access,
    FR_Access,
    MUL,
    MULS,
    Register_BITOP_Rs,
    Register_BITOP_offset,
    Memory_BITOP_offset,
    Memory_BITOP_Rs,
    sixteen_bits_Shift,
    RETI,
    RETF,
    Base_plus_Disp6,
    IMM6,
    Branch,
    Stack_Operation,
    DS_Indirect,
    IMM16,
    Direct16,
    Direct6,
    Register,

    InvalidInstructionType,
}

/* Associated Functions and Methods */

//TODO

/* Functions */

pub(super) fn decode(inst_word: u16) -> DecodedInstructionType {
    unimplemented!();//TODO
}
