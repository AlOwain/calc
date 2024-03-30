use crate::token::*;

// FIXME: Replace me with a macro
fn err_unexpected_token(word: &String) {
    println!("calc: Unexpected token: {}", word);
    std::process::exit(1);
}

pub fn tokenizer(args: Vec<String>) -> Vec<Token> {
    let mut statement: Vec<Token> = Vec::new();

    for arg in args.iter() {
        let mut value = String::new();
        let mut token = Token::None;
        for byte in arg.as_bytes() {
            match byte {
                43 => {
                    if !value.is_empty() {
                        statement.push(Token::Operand(Operand::new(value)));
                        token = Token::None;
                        value = String::new();
                    }
                    if !matches!(token, Token::None) {
                        err_unexpected_token(arg);
                    }

                    token = Token::Operator(Operator::Add);
                    statement.push(token);
                    token = Token::None
                }
                48..=57 => {
                    if matches!(token, Token::None | Token::Operand(Operand::None)) {
                        if matches!(token, Token::None) {
                            token = Token::Operand(Operand::None);
                        }
                        value.push(*byte as char);
                    }
                    else {
                        println!("I am here, {}", *byte as char);
                        err_unexpected_token(arg);
                    }
                }
                _ => {
                    // FIXME: Replace me with a macro too!
                    println!("calc: Invalid token: \'{}\'", arg);
                    std::process::exit(1);
                }
            }
        }
        if matches!(token, Token::Operand(_)) {
            statement.push(Token::Operand(Operand::new(value)))
        }
    }
    statement
}

pub fn lexer(args: Vec<String>) -> Vec<Token> {
    tokenizer(args)
}
