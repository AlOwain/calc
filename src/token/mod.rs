pub mod operand;
pub mod operator;

use std::fmt;

#[derive(Debug)]
pub enum Token {
    Operand(operand::Operand),
    Operator(operator::Operator),
    None
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
            _ => todo!("Failed to display unimplemented Token type; only Operand and Operator Tokens can be displayed")
        }
    }
}

pub use operator::Operator;
pub use operand::Operand;
pub use operand::Numeric;
