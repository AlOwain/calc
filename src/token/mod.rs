pub mod operand;
pub mod operator;

use std::fmt;

#[derive(Debug, PartialEq)]
pub enum Token {
    Operand(operand::Operand),
    Operator(operator::Operator),
}

impl fmt::Display for Token {
    fn fmt(&self, format: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Token::Operand(operand) => {
                write!(format, "{}", format!("{}", operand))
            }
            Token::Operator(operator) => {
                write!(format, "{}", format!("{}", operator))
            }
        }
    }
}

pub use operator::Operator;
pub use operand::Operand;
