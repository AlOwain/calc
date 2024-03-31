use crate::token::*;
use crate::err;

pub fn tokenizer(args: Vec<String>) -> Vec<Token> {
    let mut statement: Vec<Token> = Vec::new();

    for arg in args.iter() {
        let mut value = String::new();
        let mut token = Token::None;
        for byte in arg.as_bytes() {
            match byte {
                43 => {
                    if !value.is_empty() {
                        statement.push(Token::Operand(Operand::from(value)));
                        token = Token::None;
                        value = String::new();
                    }
                    if !matches!(token, Token::None) { err!("Error while parsing, invalid token state: {}, {:?}", arg, token); }

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
                    else { err!("Unexpected token: {}", arg); }
                }
                _ => { err!("Invalid token: \'{}\'", arg); }
            }
        }
        if matches!(token, Token::Operand(_)) {
            statement.push(Token::Operand(Operand::from(value)))
        }
    }
    statement
}

pub fn lexer(args: Vec<String>) -> Vec<Token> {
    tokenizer(args)
}
