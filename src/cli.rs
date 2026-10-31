use crate::engine::eval::{EvalErr, eval};
use crate::engine::parser::{ParseError, parse};
use crate::engine::tokenizer::{TokenizeError, tokenize};
use clap::ValueEnum;

pub mod args;
pub mod repl;

#[derive(Debug, Clone, ValueEnum)]
pub enum Mode {
    Eval,
    Tokens,
    Ast,
}

#[derive(Debug)]
pub enum AppError {
    Tokenize(TokenizeError),
    Parse(ParseError),
    Eval(EvalErr),
}

impl From<ParseError> for AppError {
    fn from(e: ParseError) -> Self {
        AppError::Parse(e)
    }
}
impl From<EvalErr> for AppError {
    fn from(e: EvalErr) -> Self {
        AppError::Eval(e)
    }
}
impl From<TokenizeError> for AppError {
    fn from(e: TokenizeError) -> Self {
        AppError::Tokenize(e)
    }
}
impl AppError {
    /// 带源码上下文的友好渲染（给 REPL 用）
    pub fn render(&self, source: &str) -> String {
        match self {
            AppError::Tokenize(e) => render_tokenize(e, source),
            AppError::Parse(e) => render_parse(e, source),
            AppError::Eval(e) => e.format_friendly(),
        }
    }
}

fn render_parse(e: &ParseError, source: &str) -> String {
    let (pos, msg) = e.describe();
    format!(
        "parse error at position {pos}: {msg}\n{}",
        arrow(source, pos)
    )
}

fn render_tokenize(e: &TokenizeError, source: &str) -> String {
    let (pos, msg) = e.describe();
    format!(
        "tokenize error at position {pos}: {msg}\n{}",
        arrow(source, pos)
    )
}

fn arrow(source: &str, pos: usize) -> String {
    let col = source.chars().take(pos).count();
    format!("  {source}\n  {}^", " ".repeat(col))
}

pub fn run_once(expr: &str, mode: Mode) -> Result<(), AppError> {
    match mode {
        Mode::Eval => {
            let tokens = tokenize(expr)?;
            let tree = parse(tokens)?;
            let result = eval(tree)?;
            println!("{}", result);
        }
        Mode::Tokens => {
            let tokens = tokenize(expr)?;
            println!("{:#?}", tokens);
        }
        Mode::Ast => {
            let tokens = tokenize(expr)?;
            let tree = parse(tokens)?;
            println!("{:#?}", tree);
        }
    }
    Ok(())
}
