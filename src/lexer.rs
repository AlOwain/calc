use crate::token::*;
use crate::err;

macro_rules! flush {
    ($operand: ident, $statement: ident) => {
        if $operand != "" {
            match Operand::from_string(&$operand) {
                Ok(operand) => {
                    match operand {
                        Some(val) => $statement.push(Token::Operand(val)),
                        _ => ()
                    }
                }
                Err(err) => err!("Parsing Operand \'{}\' failed.\n{}", $operand, err)
            }
            $operand = String::new();
        }
    };
}

pub fn lexer(args: Vec<String>) -> Vec<Token> {
    let mut statement: Vec<Token> = Vec::new();

    let mut curr_op = String::new();
    for word in args.iter() {
        for character in word.chars() {
            match &character {
                ' ' => flush!(curr_op, statement),
                '0'..='9' => curr_op.push(character),
                '.' => curr_op.push(character),
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
