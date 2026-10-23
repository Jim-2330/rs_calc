use std::str::FromStr;

pub use bigdecimal::BigDecimal;
pub use rs_calc::engine::eval::eval;
pub use rs_calc::engine::parser::parse;
pub use rs_calc::engine::tokenizer::tokenize;

#[test]
fn test_eval() {
    assert_eq!(
        eval(parse(tokenize("1 + 1").expect("tokenize error")).expect("parse error"))
            .expect("eval error"),
        BigDecimal::from_str("2").expect("constenet value error")
    );
    assert_eq!(
        eval(parse(tokenize("2 * 3 + (20 - 30)").expect("tokenize error")).expect("parse error"))
            .expect("eval error"),
        BigDecimal::from_str("-3.0").expect("constent value error")
    )
}
