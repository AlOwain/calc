use std::fmt;
use crate::err;

#[derive(Debug, PartialEq)]
pub enum Operand {
    Numeric(i64),
}

impl From<char> for Operand {
    fn from(val: char) -> Self {
        match val {
            '0'..='9' => Operand::Numeric(val as i64 - 48),
            _ => err!("Converting \'{}\' to Operand type unsupported", val),
        }
    }
}

impl Into<i64> for &Operand {
    fn into(self) -> i64 {
        match self {
            Operand::Numeric(val) => *val,
            _ => err!("Failed to convert Operand \'{}\'.", self),
        }
    }
}

impl fmt::Display for Operand {
    fn fmt(&self, format: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Operand::Numeric(val) => {
                write!(format, "{}", *val)
            }

            // NOTE: be careful, an infinite recursive loop may occur here.
            _ => err!("Failed to display Operand \'{:?}\'.", self),
        }
    }
}
