use crate::err;
use crate::token::*;

use std::cmp::Ordering;




fn handle_add(lhs: &Operand, rhs: &Operand) -> Operand {
    Operand::Numeric(lhs.into_i64() + rhs.into_i64())
}

fn handle_subtract(lhs: &Operand, rhs: &Operand) -> Operand {
    Operand::Numeric(lhs.into_i64() - rhs.into_i64())
}

fn handle_multiply(lhs: &Operand, rhs: &Operand) -> Operand {
    Operand::Numeric(lhs.into_i64() * rhs.into_i64())
}

fn handle_divide(lhs: &Operand, rhs: &Operand) -> Operand {
    Operand::Numeric(lhs.into_i64() / rhs.into_i64())
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
        write!(format, "{}", self.symbol())
    }
}

const REGISTRY: [(Operator, u8, char); 4]= [
    (Operator::Multiply,    2, '*'),
    (Operator::Divide,      2, '/'),
    (Operator::Add,         3, '+'),
    (Operator::Subtract,    3, '-'),
];
#[derive(Debug, PartialEq)]
pub enum Operator {
    Add,
    Subtract,
    Multiply,
    Divide,
}
impl Operator {
    pub fn order(&self) -> u8 {
        for (k, v, _) in REGISTRY.iter() { if self == k { return *v; } };
        err!("Operator \'{}\' order unknown", self);
    }
    pub fn symbol(&self) -> char {
        for (k, _, v) in REGISTRY.iter() { if self == k { return *v; } };
        err!("Operator \'{}\' symbol unknown", self);
    }

    pub fn do_operation(self, lhs: &Operand, rhs: &Operand) -> Operand {
        match self {
            Operator::Add       => handle_add(lhs, rhs),
            Operator::Subtract  => handle_subtract(lhs, rhs),
            Operator::Multiply  => handle_multiply(lhs, rhs),
            Operator::Divide    => handle_divide(lhs, rhs),

            _ => err!("Operation \'{}\' is not supported", self)
        }
    }

    pub fn cmp(&self, other: &Operator) -> Ordering {
        let self_order = self.order();
        let other_order = other.order();

        if self_order < other_order { return Ordering::Greater; }
        if self_order > other_order { return Ordering::Less; }
        Ordering::Equal
    }
}
