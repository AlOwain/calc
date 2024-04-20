use std::fmt;
use crate::err;

#[derive(Debug, PartialEq)]
pub struct Numeric {
    pub value: i64,
}

#[derive(Debug, PartialEq)]
pub enum Operand {
    Numeric(Numeric),
    None
}

impl From<char> for Operand {
    fn from(val: char) -> Self {
        match val {
            '0'..='9' => {
                Operand::Numeric(Numeric {
                    value: (val as u8 - b'0') as i64
                })
            }
            _ => err!("Converting \'{}\' to Operand type unsupported", val),
        }
    }
}

impl Into<i64> for &Operand {
    fn into(self) -> i64 {
        match self {
            Operand::Numeric(val) => val.value as i64,
            _ => todo!("Conversion into values has not been implemented yet for non-numeral operands."),
        }
    }
}

impl fmt::Display for Operand {
    fn fmt(&self, format: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Operand::Numeric(val) => {
                write!(format, "{}", val.value as i64)
            }
            _ => todo!("Failed to display non-numeric Operand type.")
        }
    }
}
