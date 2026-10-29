use bigdecimal::BigDecimal;

#[derive(Debug, PartialEq)]
pub enum Expr {
    Number(BigDecimal),
    Binary {
        op: Op,
        left: Box<Expr>,
        right: Box<Expr>,
    },
    Unary {
        op: UnaryOp,
        operand: Box<Expr>,
    },
}

#[derive(Debug, PartialEq)]
pub enum Op {
    Plus,
    Minus,
    Times,
    Divide,
    Power,
}

#[derive(Debug, PartialEq)]
pub enum UnaryOp {
    NEG,
}

#[derive(Debug, Clone, PartialEq)]
pub enum TokenKind {
    NUM,
    LPAREN,
    RPAREN,
    PLUS,
    MINUS,
    TIMES,
    DIVIDE,
    POWER,
    EOF,
}

#[derive(Clone, Debug, PartialEq)]
pub struct Token {
    pub kind: TokenKind,
    pub value: Option<BigDecimal>,
    pub pos: usize,
}
