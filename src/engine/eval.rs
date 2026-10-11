use crate::engine::ast::{Expr, Op, UnaryOp};

fn apply_op(op: Op, left: f64, right: f64) -> Result<f64, EvalErr> {
    match op {
        Op::PLUS => Ok(left + right),
        Op::MINUS => Ok(left - right),
        Op::TIMES => Ok(left * right),
        Op::DIVIDE => {
            if right == 0.0 {
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

fn apply_unary(op: UnaryOp, operand: f64) -> f64 {
    match op {
        UnaryOp::NEG => -operand,
    }
}

pub fn eval(expr: Expr) -> Result<f64, EvalErr> {
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
