use crate::engine::ast::{Expr, Op, Token, TokenKind, UnaryOp};

#[derive(Debug)]
pub enum ParseError {
    UnexpectedToken {
        expected: TokenKind,
        got: Token,
        pos: usize,
    },
    ExpectedEof {
        pos: usize,
    },
    NotAnOperator {
        got: TokenKind,
        pos: usize,
    },
}

#[derive(Clone, Debug)]
struct Parser {
    tokens: Vec<Token>,
    pos: usize,
}

impl Parser {
    fn new(toks: Vec<Token>) -> Self {
        Self {
            tokens: toks,
            pos: 0,
        }
    }
    fn peek(&self) -> Token {
        self.tokens
            .get(self.pos)
            .cloned()
            .expect("peek out of bounds")
    }
    fn advance(&mut self) -> Token {
        let tok = self.peek();
        self.pos += 1;
        tok
    }
    fn check(&self, kind: TokenKind) -> bool {
        self.peek().kind == kind
    }
    fn expect(&mut self, kind: TokenKind) -> Result<Token, ParseError> {
        let tok: Token = self.peek();
        if tok.kind != kind {
            Err(ParseError::UnexpectedToken {
                expected: kind,
                got: tok,
                pos: self.pos,
            })
        } else {
            Ok(self.advance())
        }
    }
    fn kind_to_op(&mut self, tok: TokenKind) -> Result<Op, ParseError> {
        match tok {
            TokenKind::PLUS => Ok(Op::Plus),
            TokenKind::MINUS => Ok(Op::Minus),
            TokenKind::TIMES => Ok(Op::Times),
            TokenKind::DIVIDE => Ok(Op::Divide),
            TokenKind::POWER => Ok(Op::Power),
            _ => Err(ParseError::NotAnOperator {
                got: tok,
                pos: self.pos,
            }),
        }
    }
    fn parse_expr(&mut self) -> Result<Expr, ParseError> {
        let mut left: Expr = self.parse_term()?;
        while self.check(TokenKind::PLUS) || self.check(TokenKind::MINUS) {
            let tok = self.advance();
            let op = self.kind_to_op(tok.kind)?;
            let right = self.parse_term()?;
            left = Expr::Binary {
                op,
                left: Box::new(left),
                right: Box::new(right),
            };
        }
        Ok(left)
    }
    fn parse_term(&mut self) -> Result<Expr, ParseError> {
        let mut left: Expr = self.parse_power()?;
        while self.check(TokenKind::TIMES) || self.check(TokenKind::DIVIDE) {
            let tok = self.advance();
            let op = self.kind_to_op(tok.kind)?;
            let right = self.parse_power()?;
            left = Expr::Binary {
                op,
                left: Box::new(left),
                right: Box::new(right),
            };
        }
        Ok(left)
    }
    fn parse_power(&mut self) -> Result<Expr, ParseError> {
        let mut left: Expr = self.parse_factor()?;
        while self.check(TokenKind::POWER) {
            let tok = self.advance();
            let op = self.kind_to_op(tok.kind)?;
            let right = self.parse_power()?;
            left = Expr::Binary {
                op,
                left: Box::new(left),
                right: Box::new(right),
            };
        }
        Ok(left)
    }
    fn parse_factor(&mut self) -> Result<Expr, ParseError> {
        if self.check(TokenKind::NUM) {
            let tmp = self.peek();
            self.advance();
            match tmp.value {
                Some(num) => Ok(Expr::Number(num)),
                None => unreachable!(),
            }
        } else if self.check(TokenKind::LPAREN) {
            self.advance();
            let e = self.parse_expr()?;
            self.expect(TokenKind::RPAREN)?;
            Ok(e)
        } else if self.check(TokenKind::MINUS) {
            self.advance();
            let operand = self.parse_factor();
            match operand {
                Ok(expr) => Ok(Expr::Unary {
                    op: UnaryOp::NEG,
                    operand: Box::new(expr),
                }),
                Err(err) => Err(err),
            }
        } else {
            Err(ParseError::NotAnOperator {
                got: self.peek().kind,
                pos: self.pos,
            })
        }
    }
}

pub fn parse(tokens: Vec<Token>) -> Result<Expr, ParseError> {
    let mut p = Parser::new(tokens);
    let expr = p.parse_expr()?;
    if p.peek().kind != TokenKind::EOF {
        return Err(ParseError::ExpectedEof { pos: p.pos });
    }
    Ok(expr)
}
