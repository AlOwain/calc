use crate::token::*;
use std::cmp::Ordering;

const ORDER: [Operator; 4]= [
    Operator::Multiply,
    Operator::Divide,
    Operator::Add,
    Operator::Subtract
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

// FIXME: when `Multiply` is compared with `Divide` or `Add` with `Subtract` it must be equal
// instead of being definitively larger or smaller.
impl PartialOrd for Operator {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {

        for element in ORDER.iter() {
            if self == element {
                return Some(Ordering::Greater);
            }
            if other == element {
                return Some(Ordering::Less); 
            }
        }
        None
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
            Operator::Add => {
                handle_add(lhs, rhs)
            },
            Operator::Subtract=> {
                handle_subtract(lhs, rhs)
            },
            _ => todo!("Operations on other types have not been implemented yet!")
        }
    }
}
