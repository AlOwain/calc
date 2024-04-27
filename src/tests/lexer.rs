#[cfg(test)]
use crate::lexer;
use crate::token::*;

#[test]
fn numeric_lexer() {
    let result = lexer::lexer(vec!("1391303".to_string()));
    assert_eq!(result, vec!(Token::Operand(Operand::Numeric(1391303))));
}

#[test]
fn digit_lexer() {
    let result = lexer::lexer(vec!("1".to_string()));
    assert_eq!(result, vec!(Token::Operand(Operand::Numeric(1))));
}
