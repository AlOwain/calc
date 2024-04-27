#[cfg(test)]
use crate::evaluator::*;
use crate::token::*;

#[test]
fn operator_push() {
    let mut stack: Vec<Operator> = Vec::new();
    let mut statement: Vec<Token> = vec!(
        Token::Operand(Operand::Numeric(3)),
    );
    push_op(Operator::Add, &mut stack, &mut statement);

    let statement_post = vec!(
        Token::Operand(Operand::Numeric(3)),
    );
    let stack_post = vec!(
        Operator::Add,
    );
    assert_eq!(statement, statement_post);
    assert_eq!(stack, stack_post);
}

#[test]
fn postfix() {
    let equation = vec!(
        Token::Operand(Operand::Numeric(3)),
        Token::Operator(Operator::Add),
        Token::Operand(Operand::Numeric(4)),
        Token::Operator(Operator::Multiply),
        Token::Operand(Operand::Numeric(5)),
    );

    let postfixed = vec!(
        Token::Operand(Operand::Numeric(3)),
        Token::Operand(Operand::Numeric(4)),
        Token::Operand(Operand::Numeric(5)),
        Token::Operator(Operator::Multiply),
        Token::Operator(Operator::Add),
    );
    assert_eq!(to_postfix(equation), postfixed)
}

// To generate similar test cases, use the following snippet:
// https://gist.github.com/AlOwain/1885b961a219a483e7d030f0929e8369
#[test]
fn multi_digit_evaluator() {
    let equation = vec!(
        Token::Operand(Operand::Numeric(16672)),
        Token::Operator(Operator::Multiply),
        Token::Operand(Operand::Numeric(94193)),
    );
    assert_eq!(evaluate(equation), 1570385696);
}
#[test]
fn evaluator() {
    let equation = vec!(
        Token::Operand(Operand::Numeric(6)),
        Token::Operator(Operator::Subtract),
        Token::Operand(Operand::Numeric(3)),
    );
    assert_eq!(evaluate(equation), 3);
}
