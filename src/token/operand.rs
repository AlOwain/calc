#[derive(Debug)]
struct Numeric {
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

impl Operand {
    pub fn value(self) -> f64 {
        match self {
            Self::Numeric(val) => val.value as f64,
            _ => panic!()
        }
    }
}
