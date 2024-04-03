use std::fmt;

#[derive(Debug)]
pub struct Numeric {
    pub value: i64,
}

#[derive(Debug)]
pub enum Operand {
    Numeric(Numeric),
    None
}

impl From<String> for Operand {
    fn from(val: String) -> Self {
        let val = val.parse().expect("calc: Unexpected value given as operand \'{value}\'");
        Operand::Numeric(Numeric {
            value: val,
        })
    }
}

impl From<char> for Operand {
    fn from(val: char) -> Self {
        match val {
            '0'..='9' => {
                Operand::Numeric(Numeric {
                    value: (val as u8 - b'0') as i64
                })
            }
            _ => todo!("Operands could not be created from non-numeral character \'{}\'", val),
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
