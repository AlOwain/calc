#[derive(Debug)]
pub struct Numeric {
    value: i64,
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

impl Into<i64> for Operand {
    fn into(self) -> i64 {
        match self {
            Self::Numeric(val) => val.value as i64,
            _ => todo!("Other operand types have not been implemented yet!")
        }
    }
}
