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
    fn from(value: String) -> Self {
        let value = value.parse().expect("calc: Unexpected value given as operand \'{value}\'");
        Operand::Numeric(Numeric {
            value,
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
            // FIXME: Handle non-numeral cases
            _ => todo!("Operands could not be created from non-numeric character \'{}\'", val),
        }
    }
}

impl Into<i64> for Operand {
    fn into(self) -> i64 {
        match self {
            Self::Numeric(val) => val.value as i64,
            _ => todo!("Conversion into values has not been implemented yet for non-numeric operands.")
        }
    }
}
