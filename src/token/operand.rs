use std::{fmt, num::ParseFloatError};
use crate::err;

#[derive(Debug, PartialEq)]
pub enum Operand {
    Numeric(i64),
    Decimal(f64),
}
impl Operand {
    pub fn into_i64(&self) -> i64 {
        match self {
            Operand::Numeric(val) => *val,
            Operand::Decimal(val) => *val as i64,
        }
    }

    pub fn from_string(string: &str) -> Result<Option<Self>, ParseFloatError> {
        match string.parse::<f64>() {
            Ok(value) => {
                if value == 0.0 { return Ok(None) }
                if value.fract() == 0.0 {
                    return Ok(Some(Operand::Numeric(value as i64)));
                }
                Ok(Some(Operand::Decimal(value)))
            },
            Err(err) => Err(err),
        }
    }

    #[allow(unreachable_patterns)]
    pub fn to_string(&self) -> String {
        match self {
            Operand::Numeric(val) => val.to_string(),
            Operand::Decimal(val) => val.to_string(),

            // NOTE: be careful, an infinite recursive loop may occur here.
            _ => err!("Stringinfying Operand \'{:?}\' failed.", self),
        }
    }
}

#[allow(unreachable_patterns)]
impl fmt::Display for Operand {
    fn fmt(&self, format: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Operand::Numeric(_) => {
                write!(format, "{}", self.to_string())
            }
            Operand::Decimal(_) => {
                write!(format, "{}", self.to_string())
            }

            // NOTE: be careful, an infinite recursive loop may occur here.
            _ => err!("Failed to display Operand \'{:?}\'.", self),
        }
    }
}
