use crate::token::*;
use std::cmp::Ordering;

const ORDER: [(u8, Operator); 4]= [
    (2, Operator::Multiply),
    (2, Operator::Divide),
    (3, Operator::Add),
    (3, Operator::Subtract),
];
#[derive(Debug, PartialEq)]
pub enum Operator {
    Add,
    Subtract,
    Multiply,
    Divide,
    None,
}

fn handle_add(lhs: &Operand, rhs: &Operand) -> Operand {
    let lhs: i64 = lhs.into();
    let rhs: i64 = rhs.into();

    let ans = lhs + rhs;
    Operand::Numeric(Numeric {
        value: ans,
    })
}

fn handle_subtract(lhs: &Operand, rhs: &Operand) -> Operand {
    let lhs: i64 = lhs.into();
    let rhs: i64 = rhs.into();

    let ans = lhs - rhs;
    Operand::Numeric(Numeric {
        value: ans,
    })
}

fn handle_multiply(lhs: &Operand, rhs: &Operand) -> Operand {
    let lhs: i64 = lhs.into();
    let rhs: i64 = rhs.into();

    let ans = lhs * rhs;
    Operand::Numeric(Numeric {
        value: ans,
    })
}

fn handle_divide(lhs: &Operand, rhs: &Operand) -> Operand {
    let lhs: i64 = lhs.into();
    let rhs: i64 = rhs.into();

    let ans = lhs / rhs;
    Operand::Numeric(Numeric {
        value: ans,
    })
}

// FIXME: when `Multiply` is compared with `Divide` or `Add` with `Subtract` it must be equal
// instead of being definitively larger or smaller.
impl PartialOrd for Operator {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl fmt::Display for Operator {
    fn fmt(&self, format: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Operator::Add       => write!(format, "+"),
            Operator::Subtract  => write!(format, "-"),
            Operator::Multiply  => write!(format, "*"),
            Operator::Divide    => write!(format, "/"),
            _ => todo!("Failed to display non-numeric Operand type."),
        }
    }
}

impl Operator {
    pub fn do_operation(self, lhs: &Operand, rhs: &Operand) -> Operand {
        match self {
            Operator::Add       => handle_add(lhs, rhs),
            Operator::Subtract  => handle_subtract(lhs, rhs),
            Operator::Multiply  => handle_multiply(lhs, rhs),
            Operator::Divide    => handle_divide(lhs, rhs),
            _ => todo!("Operations on other types have not been implemented yet!")
        }
    }

    pub fn cmp(&self, other: &Operator) -> Ordering {
        let mut self_value: u8 = 0;
        for (key, value) in ORDER.iter() {
            if self == value { self_value = *key }
        };
        let mut other_value: u8 = 0;
        for (key, value) in ORDER.iter() {
            if other == value { other_value = *key; }
        };

        if self_value < other_value { return Ordering::Greater; }
        if self_value > other_value { return Ordering::Less; }
        Ordering::Equal
    }
}
