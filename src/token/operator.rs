use crate::err;
use crate::token::*;

use std::cmp::Ordering;

pub fn get_order(op: &Operator) -> u8 {
    for (k, v) in ORDER.iter() { if op == v { return *k; } };
    err!("Operator \'{}\' order not known", op);
}
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
}

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
        match self {
            Operator::Add       => write!(format, "+"),
            Operator::Subtract  => write!(format, "-"),
            Operator::Multiply  => write!(format, "*"),
            Operator::Divide    => write!(format, "/"),

            // NOTE: be careful, an infinite recursive loop may occur here.
            _ => err!("Failed to display Operator \'{:?}\'.", self),
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

            _ => err!("Operation \'{}\' is not supported", self)
        }
    }

    pub fn cmp(&self, other: &Operator) -> Ordering {
        let self_order = get_order(self);
        let other_order = get_order(other);

        if self_order < other_order { return Ordering::Greater; }
        if self_order > other_order { return Ordering::Less; }
        Ordering::Equal
    }
}
