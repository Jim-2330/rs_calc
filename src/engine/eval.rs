use std::str::FromStr;

use crate::engine::ast::{Expr, Op, UnaryOp};
use bigdecimal::BigDecimal;

fn apply_op(op: Op, left: BigDecimal, right: BigDecimal) -> Result<BigDecimal, EvalErr> {
    match op {
        Op::Plus => Ok(left + right),
        Op::Minus => Ok(left - right),
        Op::Times => Ok(left * right),
        Op::Divide => {
            if right == BigDecimal::from_str("0.0").unwrap() {
                Err(EvalErr::DivideByZero)
            } else {
                Ok(left / right)
            }
        }
    }
}

#[derive(Debug)]
pub enum EvalErr {
    DivideByZero,
}

fn apply_unary(op: UnaryOp, operand: BigDecimal) -> BigDecimal {
    match op {
        UnaryOp::NEG => -operand,
    }
}

pub fn eval(expr: Expr) -> Result<BigDecimal, EvalErr> {
    match expr {
        Expr::Number(v) => Ok(v),
        Expr::Binary { op, left, right } => {
            let l = eval(*left)?;
            let r = eval(*right)?;
            Ok(apply_op(op, l, r)?)
        }
        Expr::Unary { op, operand } => {
            let v = eval(*operand)?;
            Ok(apply_unary(op, v))
        }
    }
}
