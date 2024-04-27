use std::cmp::Ordering;

use crate::token::*;
use crate::err;

pub fn push_op(operator: Operator, stack: &mut Vec<Operator>, statement: &mut Vec<Token>) {
    // FIXME:   If the operator is a right parantheses empty the stack up to the left
    //          bracket and push it into the statement
    match stack.last() {
        Some(stack_top) => {
            if operator.cmp(&stack_top) == Ordering::Greater {
                stack.push(operator);
                return;
            }
            statement.push(Token::Operator(stack.pop().unwrap()));
            push_op(operator, stack, statement);
        }
        None => stack.push(operator),
    }
}

pub fn to_postfix(tokens: Vec<Token>) -> Vec<Token> {
    let mut statement: Vec<Token> = Vec::with_capacity(tokens.len());
    let mut stack: Vec<Operator> = Vec::new();
    for token in tokens {
        match token {
            Token::Operand(_) => statement.push(token),
            Token::Operator(operator) => {
                push_op(operator, &mut stack, &mut statement);
            }
        }
    }

    // NOTE:       The stack must be reversed before it is appended,
    //          as the elements of the stack are popped of the end
    //          and not appended such that that first element is added
    //          before the rest.
    //
    //              Reversing the vector is not an ideal solution performance-wise,
    //          but it will do for now.
    while !stack.is_empty() {
        statement.push(Token::Operator(stack.pop().unwrap()));
    }
    statement
}

fn solve(problem: &mut Vec<Token>) -> Operand {
    let val = match problem.pop() {
        Some(value) => value,
        None => err!("Additional operator provided!"),
    };

    match val {
        Token::Operand(re) => re,
        Token::Operator(op) => {
            let rhs = solve(problem);
            let lhs = solve(problem);
            op.operate(&lhs, &rhs)
        },
    }

}

pub fn evaluate(operation: Vec<Token>) -> i64 {
    let mut operation = to_postfix(operation);
    solve(&mut operation).into_i64()
}
