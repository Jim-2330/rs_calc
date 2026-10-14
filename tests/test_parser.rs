pub use rs_calc::engine::ast::{Expr, Op, UnaryOp};
pub use rs_calc::engine::parser::parse;
pub use rs_calc::engine::tokenizer::tokenize;

#[test]
fn test_parser() {
    assert_eq!(
        parse(tokenize("1 + 1").expect("tokenize error")).expect("parse error"),
        Expr::Binary {
            op: Op::Plus,
            left: Box::new(Expr::Number(1.0,)),
            right: Box::new(Expr::Number(1.0,)),
        }
    );
    assert_eq!(
        parse(tokenize("3 * (5 +8)").expect("tokenize error")).expect("parse error"),
        Expr::Binary {
            op: Op::Times,
            left: Box::new(Expr::Number(3.0,)),
            right: Box::new(Expr::Binary {
                op: Op::Plus,
                left: Box::new(Expr::Number(5.0,)),
                right: Box::new(Expr::Number(8.0,)),
            }),
        }
    )
}
