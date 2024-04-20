#[cfg(test)]
use std::cmp::Ordering;
use crate::token::*;

#[test]
fn order_of_operations() {
    let multiply    = Operator::Multiply;
    let divide      = Operator::Divide;
    let add         = Operator::Add;
    let subtract    = Operator::Subtract;

    assert!(multiply.cmp(&multiply) == Ordering::Equal);
    assert!(multiply.cmp(&divide)   == Ordering::Equal);
    assert!(multiply.cmp(&add)      == Ordering::Greater);
    assert!(multiply.cmp(&subtract) == Ordering::Greater);

    assert!(divide.cmp(&multiply)   == Ordering::Equal);
    assert!(divide.cmp(&divide)     == Ordering::Equal);
    assert!(divide.cmp(&add)        == Ordering::Greater);
    assert!(divide.cmp(&subtract)   == Ordering::Greater);

    assert!(add.cmp(&multiply)      == Ordering::Less);
    assert!(add.cmp(&divide)        == Ordering::Less);
    assert!(add.cmp(&add)           == Ordering::Equal);
    assert!(add.cmp(&subtract)      == Ordering::Equal);

    assert!(subtract.cmp(&multiply) == Ordering::Less);
    assert!(subtract.cmp(&divide)   == Ordering::Less);
    assert!(subtract.cmp(&add)      == Ordering::Equal);
    assert!(subtract.cmp(&subtract) == Ordering::Equal);
}

#[test]
fn operator() {
    let lhs = Operand::Numeric(1);
    let rhs = Operand::Numeric(1);
    let add = Operator::Add;
    assert_eq!(
        add.do_operation(&lhs, &rhs),
        Operand::Numeric(2)
    );
}
