use crate::token::*;
use crate::err;

fn to_postfix(tokens: Vec<Token>) -> Vec<Token> {
    let mut stack: Vec<Token> = Vec::with_capacity(tokens.len());
    let mut operators: Vec<Token> = Vec::new();
    // FIXME:       Implement the order of operations
    //          obviously it does not matter right now
    //          since all we do is + anyway
    for token in tokens {
        match &token {
            Token::Operand(_) => stack.push(token),
            Token::Operator(operation) => {
                match operation {
                    Operator::Add => {
                        operators.push(token);
                    }
                    Operator::Subtract=> {
                        operators.push(token);
                    }
                }
            }
            _ => err!("Error while parsing tokens, token type undefined."),
        }
    }

    for operator in operators {
        stack.push(operator);
    }

    stack
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
            op.do_operation(lhs, rhs)
        },
        _ => err!("Token parsing failed."),
    }

}

pub fn evaluate(operation: Vec<Token>) -> i64 {
    let mut operation = to_postfix(operation);
    solve(&mut operation).into()
}
