#[cfg(test)]
use crate::token::*;

#[test]
fn order_of_operations() {
    let multiply    = Operator::Multiply;
    let divide      = Operator::Divide;
    let add         = Operator::Add;
    let subtract    = Operator::Subtract;

    assert!(multiply == multiply);
    assert!(multiply == divide);
    assert!(multiply > add);
    assert!(multiply > subtract);

    assert!(divide == multiply);
    assert!(divide == divide);
    assert!(divide > add);
    assert!(divide > subtract);

    assert!(add < multiply);
    assert!(add < divide);
    assert!(add == add);
    assert!(add == subtract);

    assert!(subtract < multiply);
    assert!(subtract < divide);
    assert!(subtract == add);
    assert!(subtract == subtract);
}

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
