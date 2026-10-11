pub use rs_calc::engine::tokenizer::{tokenize, Token};
pub use rs_calc::engine::ast::TokenKind;


#[test]
fn test_tokenize() {
    assert_eq!(
        tokenize("1 + 1"),
        [
    Token {
        kind: TokenKind::NUM,
        value: Some(
            1.0,
        ),
        pos: 0,
    },
    Token {
        kind: TokenKind::PLUS,
        value: None,
        pos: 2,
    },
    Token {
        kind: TokenKind::NUM,
        value: Some(
            1.0,
        ),
        pos: 4,
    },
    Token {
        kind: TokenKind::EOF,
        value: None,
        pos: 5,
    },
]
    );
}
