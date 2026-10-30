use crate::engine::ast::Token;
use crate::engine::ast::TokenKind;
use bigdecimal::BigDecimal;
use std::str::FromStr;

#[derive(Debug, PartialEq)]
pub enum TokenizeError {
    UnexpectedChar { ch: char, pos: usize },
}

impl TokenizeError {
    pub fn format_friendly(&mut self) -> String {
        let (pos, message) = match self {
            TokenizeError::UnexpectedChar { ch, pos } => {
                (*pos, format!("Unexpected charactor\"{}\"", ch))
            }
        };
        format!("evaluate error at position {}\n {}", pos, message)
    }
}

pub fn tokenize(s: &str) -> Result<Vec<Token>, TokenizeError> {
    let mut tokens = Vec::new();
    let chars: Vec<char> = s.chars().collect();
    let mut i = 0;
    while chars.len() > i {
        match chars[i] {
            ' ' | '\t' | '\n' => i += 1,
            d if d.is_ascii_digit() => {
                let tmp: usize = i;
                while i < chars.len() && chars[i].is_ascii_digit() {
                    i += 1;
                }
                if i < chars.len() && chars[i] == '.' {
                    i += 1;
                    while i < chars.len() && chars[i].is_ascii_digit() {
                        i += 1;
                    }
                }
                let text: String = chars[tmp..i].iter().collect();
                let value: BigDecimal =
                    BigDecimal::from_str(&text).map_err(|_| TokenizeError::UnexpectedChar {
                        ch: chars.get(i).copied().unwrap_or('\0'),
                        pos: tmp,
                    })?;
                tokens.push(Token {
                    kind: TokenKind::NUM,
                    value: Some(value),
                    pos: tmp,
                });
            }
            '(' => {
                tokens.push(Token {
                    kind: TokenKind::LPAREN,
                    value: None,
                    pos: i,
                });
                i += 1;
            }
            ')' => {
                tokens.push(Token {
                    kind: TokenKind::RPAREN,
                    value: None,
                    pos: i,
                });
                i += 1;
            }
            '+' => {
                tokens.push(Token {
                    kind: TokenKind::PLUS,
                    value: None,
                    pos: i,
                });
                i += 1;
            }
            '-' => {
                tokens.push(Token {
                    kind: TokenKind::MINUS,
                    value: None,
                    pos: i,
                });
                i += 1;
            }
            '*' => {
                tokens.push(Token {
                    kind: TokenKind::TIMES,
                    value: None,
                    pos: i,
                });
                i += 1;
            }
            '/' => {
                tokens.push(Token {
                    kind: TokenKind::DIVIDE,
                    value: None,
                    pos: i,
                });
                i += 1;
            }
            _ => {
                return Err(TokenizeError::UnexpectedChar {
                    ch: chars[i],
                    pos: i,
                });
            }
        }
    }
    tokens.push(Token {
        kind: TokenKind::EOF,
        value: None,
        pos: i,
    });
    Ok(tokens)
}
