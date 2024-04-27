#[cfg(test)]
use crate::token::*;

#[test]
fn operand() {
    let operand = Operand::Numeric(1);
    let conversion: i64 = operand.into_i64();
    assert_eq!(conversion, 1);
}
