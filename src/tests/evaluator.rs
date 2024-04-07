#[cfg(test)]
use crate::evaluator;
use crate::token::*;

#[test]
fn evaluator() {
    let equation = vec!(
        Token::Operand(Operand::Numeric(Numeric {
            value: 6,
        })),
        Token::Operator(Operator::Subtract),
        Token::Operand(Operand::Numeric(Numeric {
            value: 3,
        })),
    );
    assert_eq!(evaluator::evaluate(equation), 3);
}
