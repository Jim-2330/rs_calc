pub use rs_calc::engine::ast::{Token, TokenKind};
pub use rs_calc::engine::tokenizer::tokenize;

#[test]
fn test_tokenize() {
    assert_eq!(
        tokenize("1 + 1").unwrap_or_default(),
        [
            Token {
                kind: TokenKind::NUM,
                value: Some(1.0),
                pos: 0,
            },
            Token {
                kind: TokenKind::PLUS,
                value: None,
                pos: 2,
            },
            Token {
                kind: TokenKind::NUM,
                value: Some(1.0),
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
                value: Some(3.0),
                pos: 0,
            },
            Token {
                kind: TokenKind::TIMES,
                value: None,
                pos: 2,
            },
            Token {
                kind: TokenKind::NUM,
                value: Some(4.0),
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
                value: Some(1.0),
                pos: 9
            },
            Token {
                kind: TokenKind::PLUS,
                value: None,
                pos: 11
            },
            Token {
                kind: TokenKind::NUM,
                value: Some(1.0),
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
