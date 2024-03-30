use crate::Token;
use crate::Operator;
use crate::Operand;
// use crate::Numeric;

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
                }
            }
            _ => {
                panic!("calc: Error while parsing token");
            }
        }
    }

    for operator in operators {
        stack.push(operator);
    }

    stack
}

fn solve(operation: &mut Vec<Token>) -> Operand {
    let val = operation.pop().expect("Hello");
    match val {
        Token::Operand(re) => re,
        Token::Operator(_) => {
            todo!("Implement proper operation");
            // let rhs = solve(operation);
            // let lhs = solve(operation);
            // Operand::Numeric(Numeric {
            //     value: 34
            // })
        },
        _ => panic!("what happened?"),
    }

}

pub fn evaluate(operation: Vec<Token>) -> i64 {
    let mut operation = to_postfix(operation);
    solve(&mut operation);
    todo!("Parse operation result!");
}
