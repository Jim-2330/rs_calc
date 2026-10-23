use bigdecimal::BigDecimal;
pub use rs_calc::engine::ast::{Token, TokenKind};
pub use rs_calc::engine::tokenizer::tokenize;
use std::str::FromStr;

#[test]
fn test_tokenize() {
    assert_eq!(
        tokenize("1 + 1").expect("tokenize error"),
        [
            Token {
                kind: TokenKind::NUM,
                value: Some(BigDecimal::from_str("1").expect("constent value error")),
                pos: 0,
            },
            Token {
                kind: TokenKind::PLUS,
                value: None,
                pos: 2,
            },
            Token {
                kind: TokenKind::NUM,
                value: Some(BigDecimal::from_str("1").expect("constent value error")),
                pos: 4,
            },
            Token {
                kind: TokenKind::EOF,
                value: None,
                pos: 5,
            },
        ]
    );
    assert_eq!(
        tokenize("3 * 4 + (1 + 1)").unwrap_or_default(),
        [
            Token {
                kind: TokenKind::NUM,
                value: Some(BigDecimal::from_str("3").expect("constent value error")),
                pos: 0,
            },
            Token {
                kind: TokenKind::TIMES,
                value: None,
                pos: 2,
            },
            Token {
                kind: TokenKind::NUM,
                value: Some(BigDecimal::from_str("4").expect("constent value error")),
                pos: 4,
            },
            Token {
                kind: TokenKind::PLUS,
                value: None,
                pos: 6,
            },
            Token {
                kind: TokenKind::LPAREN,
                value: None,
                pos: 8
            },
            Token {
                kind: TokenKind::NUM,
                value: Some(BigDecimal::from_str("1").expect("constent value error")),
                pos: 9
            },
            Token {
                kind: TokenKind::PLUS,
                value: None,
                pos: 11
            },
            Token {
                kind: TokenKind::NUM,
                value: Some(BigDecimal::from_str("1").expect("constent value error")),
                pos: 13
            },
            Token {
                kind: TokenKind::RPAREN,
                value: None,
                pos: 14
            }
        ]
    );
}
