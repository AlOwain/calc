use crate::token::*;
use crate::err;

fn flush(value: String, statement: &mut Vec<Token>) -> (String, Token) {
    if !value.is_empty() {
        statement.push(Token::Operand(Operand::from(value)));
    }
    (String::new(), Token::None)
}

pub fn lexer(args: Vec<String>) -> Vec<Token> {
    let mut statement: Vec<Token> = Vec::new();

    let mut value = String::new();
    let mut token = Token::None;
    for arg in args.iter() {
        for byte in arg.as_bytes() {
            match byte {
                43 => {
                    (value, token) = flush(value, &mut statement);
                    statement.push(Token::Operator(Operator::Add));
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
        (value, token) = flush(value, &mut statement);
    }
    statement
}
