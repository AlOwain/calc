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

    pub fn from_string(string: String) -> Result<Self, String> {
        match string.parse() {
            Ok(value) => Ok(Operand::Numeric(value)),
            Err(x) => Err(format!("Failed to convert \'{}\' into Operand\n{}", string, x.to_string()))
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
