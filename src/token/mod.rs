pub mod operand;
pub mod operator;

#[derive(Debug)]
pub enum Token {
    Operand(operand::Operand),
    Operator(operator::Operator),
    None
}

pub use operator::Operator;
pub use operand::Operand;
