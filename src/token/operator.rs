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

const REGISTRY: [(Operator, u8, char, fn(&Operand, &Operand) -> Operand); 4]= [
    (Operator::Multiply,    2, '*', handle_multiply),
    (Operator::Divide,      2, '/', handle_divide),
    (Operator::Add,         3, '+', handle_add),
    (Operator::Subtract,    3, '-', handle_subtract),
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
        for (k, v, _, _) in REGISTRY.iter() { if self == k { return *v; } };
        err!("Operator \'{}\' order unknown", self);
    }
    pub fn symbol(&self) -> char {
        for (k, _, v, _) in REGISTRY.iter() { if self == k { return *v; } };
        err!("Operator \'{}\' symbol unknown", self);
    }
    pub fn handler(&self) -> fn(&Operand, &Operand) -> Operand {
        for (k, _, _, v) in REGISTRY.iter() { if self == k { return *v; } };
        err!("Operator \'{}\' handler unknown", self);
    }

    pub fn do_operation(self, lhs: &Operand, rhs: &Operand) -> Operand {
        self.handler()(lhs, rhs)
    }

    pub fn cmp(&self, other: &Operator) -> Ordering {
        let self_order = self.order();
        let other_order = other.order();

        if self_order < other_order { return Ordering::Greater; }
        if self_order > other_order { return Ordering::Less; }
        Ordering::Equal
    }
}
