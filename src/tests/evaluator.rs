#[cfg(test)]
use crate::evaluator::*;
use crate::token::*;

#[test]
fn operator_push() {
    let mut stack: Vec<Token> = Vec::new();
    let mut statement: Vec<Token> = vec!(
        Token::Operand(Operand::Numeric(Numeric {value: 3})),
    );
    push_op(Operator::Add, &mut statement, &mut stack);

    let statement_post = vec!(
        Token::Operand(Operand::Numeric(Numeric {value: 3})),
    );
    let stack_post = vec!(
        Token::Operator(Operator::Add),
    );
    assert_eq!(statement, statement_post);
    assert_eq!(stack, stack_post);
}

#[test]
fn postfix() {
    let equation = vec!(
        Token::Operand(Operand::Numeric(Numeric {value: 3})),
        Token::Operator(Operator::Add),
        Token::Operand(Operand::Numeric(Numeric {value: 4})),
        Token::Operator(Operator::Multiply),
        Token::Operand(Operand::Numeric(Numeric {value: 5})),
    );

    let postfixed = vec!(
        Token::Operand(Operand::Numeric(Numeric {value: 3})),
        Token::Operand(Operand::Numeric(Numeric {value: 4})),
        Token::Operand(Operand::Numeric(Numeric {value: 5})),
        Token::Operator(Operator::Multiply),
        Token::Operator(Operator::Add),
    );
    assert_eq!(to_postfix(equation), postfixed)
}

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
    assert_eq!(evaluate(equation), 3);
}
