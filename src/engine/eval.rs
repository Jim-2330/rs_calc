use std::str::FromStr;

use crate::engine::ast::{Expr, Op, UnaryOp};
use bigdecimal::{BigDecimal, ToPrimitive};

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
        Op::Power => high_precision_pow(&right, &left),
    }
}

fn taylor_exp(r: &BigDecimal, max_iterations: usize) -> BigDecimal {
    let mut sum = BigDecimal::from(1);
    let mut term = BigDecimal::from(1);

    for k in 1..=max_iterations {
        term = &term * r / BigDecimal::from(k as i64);
        sum = &sum + &term;

        if term == BigDecimal::from(0) {
            break;
        }
    }
    sum
}

fn high_precision_ln(x: &BigDecimal) -> Result<BigDecimal, EvalErr> {
    if x <= &BigDecimal::from(0) {
        return Err(EvalErr::PowerByNagative);
    }

    let mut y = BigDecimal::from(0);

    for _ in 0..30 {
        let exp_y = taylor_exp(&y, 50);
        let scale = 30;
        let numerator = (x - &exp_y).with_scale(scale);
        let denominator = exp_y.with_scale(scale);
        let delta = &numerator / &denominator;
        y = &y + &delta;

        if delta == BigDecimal::from(0) {
            break;
        }
    }
    Ok(y)
}

fn high_precision_pow(base: &BigDecimal, exponent: &BigDecimal) -> Result<BigDecimal, EvalErr> {
    if let Some(exp_i64) = exponent.to_i64() {
        if BigDecimal::from(exp_i64) == *exponent {
            return Ok(base.powi(exp_i64));
        }
    }

    let ln_a = high_precision_ln(base)?;
    let product = exponent * &ln_a;
    let result = taylor_exp(&product, 50);

    Ok(result)
}

#[derive(Debug)]
pub enum EvalErr {
    DivideByZero,
    PowerByNagative,
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
