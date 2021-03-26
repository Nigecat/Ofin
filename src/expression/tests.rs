use super::Expression;

macro_rules! expr {
    ($expr: literal) => {
        Expression::parse($expr).evaluate()
    };
}

macro_rules! expr_eq {
    ($expr: literal = $expected: literal) => {
        assert_eq!(expr!($expr), $expected as f32);
    };
}

#[test]
fn basic_addition() {
    expr_eq!("1+1" = 2);
    expr_eq!("5+3" = 8);
    expr_eq!("6+4" = 10);
}

#[test]
fn basic_subtraction() {
    // assert_eq!(expr!("11-4"), 7f32);
    // assert_eq!(expr!("6--2"), 8f32);
    // assert_eq!(expr!("-2-5"), -7f32);
}

#[test]
fn basic_multiplication() {
    expr_eq!("9*2" = 18);
    // expr_eq!("8*0.5" = 4);
    // expr_eq!("0.2*0.3" = 0.06);
}

#[test]
fn basic_division() {
    expr_eq!("1/2" = 0.5);
    expr_eq!("100/25" = 4);
    // expr_eq!("1/0" = 0);
}
