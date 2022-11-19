/* NAME//TODO
 * By: John Jekel
 *
 * TODO description
 *
*/

/* Imports */

//TODO (include "use" and "mod" here)

/* Constants */

//TODO

/* Macros */

macro_rules! reg_string {
    ($reg:expr) => {{
        let string: &str;
        {
            use crate::decode::DecodedRegister::*;
            match $reg  {
                SP => { string = "SP"; },
                R1_SR1 => { string = "R1"; },
                R2_SR2 => { string = "R2"; },
                R3_SR3 => { string = "R3"; },
                R4_SR4 => { string = "R4"; },
                BP => { string = "BP"; },
                SR => { string = "SR"; },
                PC => { string = "PC"; },

                Invalid => { string = "(invalid)"; }
            }
        }
        string
    }};
}
pub(super) use reg_string;

macro_rules! reg_string_lower {
    ($reg:expr) => {{
        let string: &str;
        {
            use crate::decode::DecodedRegister::*;
            match $reg  {
                SP => { string = "sp"; },
                R1_SR1 => { string = "r1"; },
                R2_SR2 => { string = "r2"; },
                R3_SR3 => { string = "r3"; },
                R4_SR4 => { string = "r4"; },
                BP => { string = "bp"; },
                SR => { string = "sr"; },
                PC => { string = "pc"; },

                Invalid => { string = "(invalid)"; }
            }
        }
        string
    }};
}
pub(super) use reg_string_lower;

macro_rules! reg_string_by_index {
    ($index:expr) => {{
        let string: &str;
        {
            match $index  {
                0b000 => { string = "SP"; },
                0b001 => { string = "R1"; },
                0b010 => { string = "R2"; },
                0b011 => { string = "R3"; },
                0b100 => { string = "R4"; },
                0b101 => { string = "BP"; },
                0b110 => { string = "SR"; },
                0b111 => { string = "PC"; },

                _ => { string = "(invalid)"; }
            }
        }
        string
    }};
}
pub(super) use reg_string_by_index;

macro_rules! reg_string_by_index_lower {
    ($index:expr) => {{
        let string: &str;
        {
            match $index  {
                0b000 => { string = "sp"; },
                0b001 => { string = "r1"; },
                0b010 => { string = "r2"; },
                0b011 => { string = "r3"; },
                0b100 => { string = "r4"; },
                0b101 => { string = "bp"; },
                0b110 => { string = "sr"; },
                0b111 => { string = "pc"; },

                _ => { string = "(invalid)"; }
            }
        }
        string
    }};
}
pub(super) use reg_string_by_index_lower;

macro_rules! bit_op_string {
    ($op:expr) => {{
        let string: &str;
        {
            use crate::decode::DecodedBitOp::*;
            match $op {
                TSTB => { string = "TSTB"; },
                SETB => { string = "SETB"; },
                CLRB => { string = "CLRB"; },
                INVB => { string = "INVB"; },

                Invalid => { string = "(invalid)"; }
            }
        }
        string
    }};
}
pub(super) use bit_op_string;

macro_rules! bit_op_string_lower {
    ($op:expr) => {{
        let string: &str;
        {
            use crate::decode::DecodedBitOp::*;
            match $op {
                TSTB => { string = "tstb"; },
                SETB => { string = "setb"; },
                CLRB => { string = "clrb"; },
                INVB => { string = "invb"; },

                Invalid => { string = "(invalid)"; }
            }
        }
        string
    }};
}
pub(super) use bit_op_string_lower;

macro_rules! lsft_op_string {
    ($op:expr) => {{
        let string: &str;
        {
            use crate::decode::DecodedLSFTOp::*;
            match $op {
                ASR => { string = "ASR"; },
                ASROR => { string = "ASROR"; },
                LSL => { string = "LSL"; },
                LSLOR => { string = "LSLOR"; },
                LSR => { string = "LSR"; },
                LSROR => { string = "LSROR"; },
                ROL => { string = "ROL"; },
                ROR => { string = "ROR"; },

                Invalid => { string = "(invalid)"; }
            }
        }
        string
    }};
}
pub(super) use lsft_op_string;

macro_rules! lsft_op_string_lower {
    ($op:expr) => {{
        let string: &str;
        {
            use crate::decode::DecodedLSFTOp::*;
            match $op {
                ASR => { string = "asr"; },
                ASROR => { string = "asror"; },
                LSL => { string = "lsl"; },
                LSLOR => { string = "lslor"; },
                LSR => { string = "lsr"; },
                LSROR => { string = "lsror"; },
                ROL => { string = "rol"; },
                ROR => { string = "ror"; },

                Invalid => { string = "(invalid)"; }
            }
        }
        string
    }};
}
pub(super) use lsft_op_string_lower;

macro_rules! sft_op_string {
    ($op:expr) => {{
        let string: &str;
        {
            use crate::decode::DecodedSFTOp::*;
            match $op {
                NOP => { string = "NOP"; },
                ASR => { string = "ASR"; },
                LSL => { string = "LSL"; },
                LSR => { string = "LSR"; },
                ROL => { string = "ROL"; },
                ROR => { string = "ROR"; },

                Invalid => { string = "(invalid)"; }
            }
        }
        string
    }};
}
pub(super) use sft_op_string;

macro_rules! sft_op_string_lower {
    ($op:expr) => {{
        let string: &str;
        {
            use crate::decode::DecodedSFTOp::*;
            match $op {
                NOP => { string = "nop"; },
                ASR => { string = "asr"; },
                LSL => { string = "lsl"; },
                LSR => { string = "lsr"; },
                ROL => { string = "rol"; },
                ROR => { string = "ror"; },

                Invalid => { string = "(invalid)"; }
            }
        }
        string
    }};
}
pub(super) use sft_op_string_lower;

macro_rules! stack_op_string {
    ($op:expr) => {{
        let string: &str;
        {
            use crate::decode::DecodedStackOp::*;
            match $op {
                PUSH => { string = "PUSH"; },
                POP => { string = "POP"; },

                Invalid => { string = "(invalid)"; }
            }
        }
        string
    }};
}
pub(super) use stack_op_string;

macro_rules! stack_op_string_lower {
    ($op:expr) => {{
        let string: &str;
        {
            use crate::decode::DecodedStackOp::*;
            match $op {
                PUSH => { string = "push"; },
                POP => { string = "pop"; },

                Invalid => { string = "(invalid)"; }
            }
        }
        string
    }};
}
pub(super) use stack_op_string_lower;

/* Static Variables */

//TODO

/* Types */

//TODO

/* Associated Functions and Methods */

//TODO

/* Functions */

//TODO
