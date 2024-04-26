use std::fmt;
use crate::err;

#[derive(Debug, PartialEq)]
pub enum Operand {
    Numeric(i64),
}
impl Operand {
    pub fn into_i64(&self) -> i64 {
        match self {
            Operand::Numeric(val) => *val,
        }
    }
    pub fn from_char(val: char) -> Option<Self> {
        match val {
            '0'..='9' => Some(Operand::Numeric(val as i64 - 48)),
            _ => None,
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
