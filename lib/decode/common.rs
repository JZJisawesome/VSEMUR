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

/* Static Variables */

//TODO

/* Types */

//TODO

/* Associated Functions and Methods */

//TODO

/* Functions */

//TODO
