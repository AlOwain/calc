#[cfg(test)]
use crate::token::*;

#[test]
fn operand() {
    let operand = Operand::Numeric(Numeric { value: 1 });
    let conversion: i64 = (&operand).into();
    assert_eq!(conversion, 1);
}
