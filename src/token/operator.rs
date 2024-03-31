use crate::token::*;

#[derive(Debug)]
pub enum Operator {
    Add,
    Subtract,
}

fn handle_add(lhs: Operand, rhs: Operand) -> Operand {
    let lhs: i64 = lhs.into();
    let rhs: i64 = rhs.into();

    let ans = lhs + rhs;
    Operand::Numeric(Numeric {
        value: ans,
    })
}

fn handle_subtract(lhs: Operand, rhs: Operand) -> Operand {
    let lhs: i64 = lhs.into();
    let rhs: i64 = rhs.into();

    let ans = lhs - rhs;
    Operand::Numeric(Numeric {
        value: ans,
    })
}

impl Operator {
    pub fn do_operation(self, lhs: Operand, rhs: Operand) -> Operand {
        match self {
            Operator::Add => {
                handle_add(lhs, rhs)
            },
            Operator::Subtract=> {
                handle_subtract(lhs, rhs)
            },
        }
    }
}
