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

impl fmt::Display for Operator {
    fn fmt(&self, format: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(format, "{}", self.symbol())
    }
}

const REGISTRY: [(Operator, u8, char, Option<fn(&Operand, &Operand) -> Operand>); 6]= [
    (Operator::Multiply,    3, '*', Some(handle_multiply)),
    (Operator::Divide,      3, '/', Some(handle_divide)),
    (Operator::Add,         4, '+', Some(handle_add)),
    (Operator::Subtract,    4, '-', Some(handle_subtract)),
    (Operator::LParan,      5, '(', None),
    (Operator::RParan,      5, ')', None),
];
#[derive(Debug, Clone, PartialEq)]
pub enum Operator {
    LParan,
    RParan,
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
        for (k, _, _, v) in REGISTRY.iter() { if self == k && !v.is_none() { return v.unwrap(); } };
        err!("Operator \'{}\' handler unknown", self);
    }

    pub fn from_char(val: &char) -> Option<Self> {
        for (v, _, k, _) in REGISTRY.iter() { if val == k { return Some(v.clone()); } };
        None
    }
    pub fn operate(self, lhs: &Operand, rhs: &Operand) -> Operand {
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
