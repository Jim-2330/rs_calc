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
