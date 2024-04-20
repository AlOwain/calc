#[cfg(test)]
use crate::lexer;
use crate::token::*;

#[test]
fn lexer() {
    let result = lexer::lexer(vec!("1".to_string()));
    assert_eq!(result, vec!(Token::Operand(Operand::Numeric(1))));
}
