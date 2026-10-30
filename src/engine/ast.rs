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

impl TokenKind {
    pub fn format_friendly(&self) -> &str {
        match self {
            TokenKind::NUM => "",
            TokenKind::LPAREN => "left brackets",
            TokenKind::RPAREN => "right brackets",
            TokenKind::PLUS => "plus",
            TokenKind::MINUS => "minus",
            TokenKind::TIMES => "times",
            TokenKind::DIVIDE => "divide",
            TokenKind::POWER => "power",
            TokenKind::EOF => "",
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct Token {
    pub kind: TokenKind,
    pub value: Option<BigDecimal>,
    pub pos: usize,
}

impl Token {
    pub fn format_friendly(&self) -> String {
        format!(
            "{} {}",
            self.kind.format_friendly(),
            self.value
                .as_ref()
                .map(|v| v.to_string())
                .unwrap_or_default()
        )
    }
}
