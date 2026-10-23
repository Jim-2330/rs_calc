use bigdecimal::BigDecimal;
pub use rs_calc::engine::ast::{Expr, Op, UnaryOp};
pub use rs_calc::engine::parser::parse;
pub use rs_calc::engine::tokenizer::tokenize;
use std::str::FromStr;

#[test]
fn test_parser() {
    assert_eq!(
        parse(tokenize("1 + 1").expect("tokenize error")).expect("parse error"),
        Expr::Binary {
            op: Op::Plus,
            left: Box::new(Expr::Number(
                BigDecimal::from_str("1").expect("constent value error"),
            )),
            right: Box::new(Expr::Number(
                BigDecimal::from_str("1").expect("constent value error"),
            )),
        }
    );
    assert_eq!(
        parse(tokenize("3 * (5 +8)").expect("tokenize error")).expect("parse error"),
        Expr::Binary {
            op: Op::Times,
            left: Box::new(Expr::Number(
                BigDecimal::from_str("3").expect("constent value error"),
            )),
            right: Box::new(Expr::Binary {
                op: Op::Plus,
                left: Box::new(Expr::Number(
                    BigDecimal::from_str("5").expect("constent value error"),
                )),
                right: Box::new(Expr::Number(
                    BigDecimal::from_str("8").expect("constent value error"),
                )),
            }),
        }
    )
}
