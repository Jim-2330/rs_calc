pub use rs_calc::engine::eval::eval;
pub use rs_calc::engine::parser::parse;
pub use rs_calc::engine::tokenizer::tokenize;

#[test]
fn test_eval() {
    assert_eq!(
        eval(parse(tokenize("1 + 1").expect("tokenize error")).expect("parse error"))
            .expect("eval error"),
        2.0
    );
    assert_eq!(
        eval(parse(tokenize("2 * 3 + (20 - 30)").expect("tokenize error")).expect("parse error"))
            .expect("eval error"),
        -3.0
    )
}
