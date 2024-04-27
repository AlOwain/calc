use crate::token::*;
use crate::err;

macro_rules! flush {
    ($operand: ident, $statement: ident) => {
        if !$operand.is_none() {
            $statement.push(Token::Operand($operand.take().unwrap()));
        }
    };
}

pub fn lexer(args: Vec<String>) -> Vec<Token> {
    let mut statement: Vec<Token> = Vec::new();

    let mut curr_op = None;
    for word in args.iter() {
        for character in word.chars() {
            match &character {
                ' ' => flush!(curr_op, statement),
                '0'..='9' => {
                    curr_op = match &curr_op {
                        Some(operand) => {
                            Some(Operand::Numeric(
                                (operand.into_i64() * 10) + (character as i64 - 48)
                            ))
                        }
                        None => Operand::from_char(character),
                    }
                }
                _ => {
                    match Operator::from_char(&character) {
                        Some(operator) => {
                            flush!(curr_op, statement);
                            statement.push(Token::Operator(operator))
                        }
                        None => err!("Invalid token: \'{}\'", word),
                    }
                }
            }
        }
        flush!(curr_op, statement);
    }
    statement
}
