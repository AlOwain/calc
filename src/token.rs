#[derive(Debug)]
pub enum Operator {
    Add,
}

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
        // TODO Convert 
        let value = match value.parse() {
            Ok(val) => val,
            Err(err) => {
                panic!("calc: {} '{}'\n{}",
                    "Unexpected value given as operand", value, err
                );
            }
        };
        Operand::Numeric(Numeric {
            value,
        })
    }
}

#[derive(Debug)]
pub enum Token {
    Operand(Operand),
    Operator(Operator),
    None
}
