#[derive(Debug, PartialEq)]
pub enum Expr {
    Number(f64),
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
    EOF,
}

#[derive(Clone, Debug, PartialEq)]
pub struct Token {
    pub kind: TokenKind,
    pub value: Option<f64>,
    pub pos: usize,
}
