/* NAME//TODO
 * By: John Jekel
 *
 * TODO description
 *
*/

/* Imports */

use crate::decode::*;

/* Constants */

//TODO

/* Macros */

//TODO make these functions

/* Static Variables */

//TODO

/* Types */

//TODO

/* Associated Functions and Methods */

//TODO

/* Functions */

pub(super) fn reg_string(reg: DecodedRegister) -> &'static str {
    use DecodedRegister::*;
    match reg  {
        SP => { return "SP"; },
        R1_SR1 => { return "R1"; },
        R2_SR2 => { return "R2"; },
        R3_SR3 => { return "R3"; },
        R4_SR4 => { return "R4"; },
        BP => { return "BP"; },
        SR => { return "SR"; },
        PC => { return "PC"; },

        Invalid => { return "(invalid)"; }
    }
}

pub(super) fn reg_string_lower(reg: DecodedRegister) -> &'static str {
    use DecodedRegister::*;
    match reg  {
        SP => { return "sp"; },
        R1_SR1 => { return "r1"; },
        R2_SR2 => { return "r2"; },
        R3_SR3 => { return "r3"; },
        R4_SR4 => { return "r4"; },
        BP => { return "bp"; },
        SR => { return "sr"; },
        PC => { return "pc"; },

        Invalid => { return "(invalid)"; }
    }
}

pub(super) fn reg_string_by_index(index: u8) -> &'static str {
    match index  {
        0b000 => { return "SP"; },
        0b001 => { return "R1"; },
        0b010 => { return "R2"; },
        0b011 => { return "R3"; },
        0b100 => { return "R4"; },
        0b101 => { return "BP"; },
        0b110 => { return "SR"; },
        0b111 => { return "PC"; },

        _ => { return "(invalid)"; }
    }
}

pub(super) fn reg_string_by_index_lower(index: u8) -> &'static str {
    match index  {
        0b000 => { return "sp"; },
        0b001 => { return "r1"; },
        0b010 => { return "r2"; },
        0b011 => { return "r3"; },
        0b100 => { return "r4"; },
        0b101 => { return "bp"; },
        0b110 => { return "sr"; },
        0b111 => { return "pc"; },

        _ => { return "(invalid)"; }
    }
}

pub(super) fn bit_op_string(op: DecodedBitOp) -> &'static str {
    use DecodedBitOp::*;
    match op {
        TSTB => { return "TSTB"; },
        SETB => { return "SETB"; },
        CLRB => { return "CLRB"; },
        INVB => { return "INVB"; },

        Invalid => { return "(invalid)"; }
    }
}

pub(super) fn bit_op_string_lower(op: DecodedBitOp) -> &'static str {
    use DecodedBitOp::*;
    match op {
        TSTB => { return "tstb"; },
        SETB => { return "setb"; },
        CLRB => { return "clrb"; },
        INVB => { return "invb"; },

        Invalid => { return "(invalid)"; }
    }
}

pub(super) fn lsft_op_string(op: DecodedLSFTOp) -> &'static str {
    use DecodedLSFTOp::*;
    match op {
        ASR => { return "ASR"; },
        ASROR => { return "ASROR"; },
        LSL => { return "LSL"; },
        LSLOR => { return "LSLOR"; },
        LSR => { return "LSR"; },
        LSROR => { return "LSROR"; },
        ROL => { return "ROL"; },
        ROR => { return "ROR"; },

        Invalid => { return "(invalid)"; }
    }
}

pub(super) fn lsft_op_string_lower(op: DecodedLSFTOp) -> &'static str {
    use DecodedLSFTOp::*;
    match op {
        ASR => { return "asr"; },
        ASROR => { return "asror"; },
        LSL => { return "lsl"; },
        LSLOR => { return "lslor"; },
        LSR => { return "lsr"; },
        LSROR => { return "lsror"; },
        ROL => { return "rol"; },
        ROR => { return "ror"; },

        Invalid => { return "(invalid)"; }
    }
}

pub(super) fn sft_op_string(op: DecodedSFTOp) -> &'static str {
    use DecodedSFTOp::*;
    match op {
        NOP => { return "NOP"; },
        ASR => { return "ASR"; },
        LSL => { return "LSL"; },
        LSR => { return "LSR"; },
        ROL => { return "ROL"; },
        ROR => { return "ROR"; },

        Invalid => { return "(invalid)"; }
    }
}

pub(super) fn sft_op_string_lower(op: DecodedSFTOp) -> &'static str {
    use DecodedSFTOp::*;
    match op {
        NOP => { return "nop"; },
        ASR => { return "asr"; },
        LSL => { return "lsl"; },
        LSR => { return "lsr"; },
        ROL => { return "rol"; },
        ROR => { return "ror"; },

        Invalid => { return "(invalid)"; }
    }
}

pub(super) fn stack_op_string(op: DecodedStackOp) -> &'static str {
    use DecodedStackOp::*;
    match op {
        PUSH => { return "PUSH"; },
        POP => { return "POP"; },

        Invalid => { return "(invalid)"; }
    }
}

pub(super) fn stack_op_string_lower(op: DecodedStackOp) -> &'static str {
    use DecodedStackOp::*;
    match op {
        PUSH => { return "push"; },
        POP => { return "pop"; },

        Invalid => { return "(invalid)"; }
    }
}
