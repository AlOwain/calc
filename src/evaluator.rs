use crate::token::*;
use crate::err;

pub fn push_op(operator: Operator, statement: &mut Vec<Token>, stack: &mut Vec<Token>) {
    // FIXME:   If the operator is a right parantheses empty the stack up to the left
    //          bracket and push it into the statement
    match stack.pop() {
        Some(stack_top) => {
            match stack_top {
                Token::Operator(ref stack_op) => {
                    if operator > *stack_op {
                        stack.push(stack_top);
                        stack.push(Token::Operator(operator));
                        return;
                    }
                    statement.push(stack_top);
                    push_op(operator, statement, stack);
                }
                _ => panic!("Non-Operator was added to Operator only stack")
            }
        }
        None => stack.push(Token::Operator(operator)),
    }
}

pub fn to_postfix(tokens: Vec<Token>) -> Vec<Token> {
    let mut statement: Vec<Token> = Vec::with_capacity(tokens.len());
    let mut stack: Vec<Token> = Vec::new();
    for token in tokens {
        match token {
            Token::Operand(_) => statement.push(token),
            Token::Operator(operator) => {
                push_op(operator, &mut statement, &mut stack);
            }
            _ => err!("Error while parsing tokens, token type undefined."),
        }
    }

    // TODO: Implement checks
    //          - If the stack is not empty
    //          - If the operands are not in the same order
    statement.append(&mut stack);
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
            op.do_operation(&lhs, &rhs)
        },
        _ => err!("Token parsing failed."),
    }

}

pub fn evaluate(operation: Vec<Token>) -> i64 {
    let mut operation = to_postfix(operation);
    (&solve(&mut operation)).into()
}
