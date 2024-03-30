#[derive(Debug)]
pub struct Numeric {
    value: i64,
}

#[derive(Debug)]
pub enum Operand {
    Numeric(Numeric),
    None
}

impl Operand {
    pub fn new(value: String) -> Self {
        let value = value.parse().expect("calc: Unexpected value given as operand \'{value}\'");
        Operand::Numeric(Numeric {
            value,
        })
    }

    pub fn value(self) -> f64 {
        match self {
            Self::Numeric(val) => val.value as f64,
            _ => panic!()
        }
    }
}
