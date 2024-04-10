#[cfg(test)]
use crate::evaluator;
use crate::token::*;

// To generate similar test cases, use the following snippet:
// https://gist.github.com/AlOwain/1885b961a219a483e7d030f0929e8369
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
