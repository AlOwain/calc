#[cfg(test)]
use crate::token::*;

#[test]
fn operator() {
    let lhs = Operand::Numeric(Numeric {
        value: 1,
    });
    let rhs = Operand::Numeric(Numeric {
        value: 1,
    });
    let add = Operator::Add;
    assert_eq!(
        add.do_operation(&lhs, &rhs),
        Operand::Numeric(Numeric {value: 2})
    );
}
