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

#[warn(dead_code)]
#[derive(Debug)]
pub enum AppError {
    Tokenize(String),
    Parse(String),
    Eval(String),
}

impl From<ParseError> for AppError {
    fn from(e: ParseError) -> Self {
        AppError::Parse(e.format_friendly())
    }
}

impl From<EvalErr> for AppError {
    fn from(e: EvalErr) -> Self {
        AppError::Eval(e.format_friendly())
    }
}

impl From<TokenizeError> for AppError {
    fn from(mut e: TokenizeError) -> Self {
        AppError::Tokenize(e.format_friendly())
    }
}

impl AppError {
    fn format_friendly(&self) -> &String {
        match self {
            AppError::Tokenize(msg) => msg,
            AppError::Parse(msg) => msg,
            AppError::Eval(msg) => msg,
        }
    }
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
