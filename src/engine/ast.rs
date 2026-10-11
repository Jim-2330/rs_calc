#[derive(Debug)]
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

#[derive(Debug)]
pub enum Op {
    PLUS,
    MINUS,
    TIMES,
    DIVIDE,
}

#[derive(Debug)]
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

#[derive(Clone, Debug)]
pub struct Token {
    pub kind: TokenKind,
    pub value: Option<f64>,
    pub pos: usize,
}
