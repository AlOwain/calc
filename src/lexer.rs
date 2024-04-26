use crate::token::*;
use crate::err;

macro_rules! flush_operand {
    ($operand: expr, $statement: expr) => {{
        if !matches!($operand, None) { $statement.push(Token::Operand($operand.unwrap())); }
        $operand = None
    }};
}
macro_rules! flush_both {
    ($operand: expr, $operator: expr, $statement: expr) => {{
        flush_operand!($operand, $statement);
        $statement.push(Token::Operator($operator))
    }};
}

pub fn lexer(args: Vec<String>) -> Vec<Token> {
    let mut statement: Vec<Token> = Vec::new();

    let mut curr_op = None;
    for word in args.iter() {
        for character in word.chars() {
            match character {
                ' ' => flush_operand!(curr_op, statement),
                '*' => flush_both!(curr_op, Operator::Multiply, statement),
                '/' => flush_both!(curr_op, Operator::Divide, statement),
                '-' => flush_both!(curr_op, Operator::Subtract, statement),
                '+' => flush_both!(curr_op, Operator::Add, statement),

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
                _ => { err!("Invalid token: \'{}\'", word); }
            }
        }
        flush_operand!(curr_op, statement);
    }
    statement
}
