use crate::Token;
use crate::Operator;

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

pub fn evaluate(operation: Vec<Token>) -> String {
    let operation = to_postfix(operation);
    dbg!(operation);

    todo!("Implement me!");
}
