use crate::token::*;
use crate::err;

macro_rules! flush {
    ($token:expr, $statement:expr) => {
        {
            if !matches!($token, Token::None) { $statement.push($token); }
            $token = Token::None
        }
    };
}

pub fn lexer(args: Vec<String>) -> Vec<Token> {
    let mut statement: Vec<Token> = Vec::new();

    let mut token = Token::None;
    for word in args.iter() {
        for character in word.chars() {
            match character {
                ' ' => flush!(token, statement),
                '-' => {
                    flush!(token, statement);
                    statement.push(Token::Operator(Operator::Subtract));
                }
                '+' => {
                    flush!(token, statement);
                    statement.push(Token::Operator(Operator::Add));
                }
                '0'..='9' => {
                    token = match &token {
                        Token::Operand(zy) => {
                            let op_value: i64 = zy.into();
                            Token::Operand(Operand::Numeric(Numeric {
                                value: (op_value * 10) + (character as u8 - b'0') as i64,
                            }))
                        } ,
                        Token::None => Token::Operand(Operand::from(character)),
                        _ => err!("Unexpected token: \'{}\'", word),
                    }
                }
                _ => { err!("Invalid token: \'{}\'", word); }
            }
        }
        flush!(token, statement);
    }
    statement
}
