use anyhow::{anyhow, Result};
use nom::{
    branch::alt,
    bytes::complete::{tag, take_while},
    combinator::map_res,
    multi::fold_many0,
    sequence::{pair, preceded, terminated},
    IResult,
};
use std::path::Path;

use crate::reader::read_lines;

#[derive(Debug, Clone)]
pub enum Expr {
    Scalar(isize),
    Subexpr(Box<Expr>),
    Add(Box<Expr>, Box<Expr>),
    Mul(Box<Expr>, Box<Expr>),
}

impl Expr {
    pub fn eval(&self) -> isize {
        match self {
            Expr::Scalar(v) => *v,
            Expr::Subexpr(e) => e.eval(),
            Expr::Add(a, b) => a.eval() + b.eval(),
            Expr::Mul(a, b) => a.eval() * b.eval(),
        }
    }
}

fn num(input: &str) -> IResult<&str, Expr> {
    map_res(
        take_while(|c: char| c.is_digit(10)),
        |s: &str| -> Result<Expr> { Ok(Expr::Scalar(s.parse()?)) },
    )(input)
}

mod part_a {
    use super::*;

    fn subexpr(input: &str) -> IResult<&str, Expr> {
        map_res(
            preceded(tag("("), terminated(expr, tag(")"))),
            |e| -> Result<Expr> { Ok(Expr::Subexpr(Box::new(e))) },
        )(input)
    }

    pub fn expr(input: &str) -> IResult<&str, Expr> {
        let (input, init) = alt((subexpr, num))(input)?;
        fold_many0(
            pair(alt((tag(" + "), tag(" * "))), alt((subexpr, num))),
            init,
            |a, (op, b)| {
                if op == " + " {
                    Expr::Add(Box::new(a), Box::new(b))
                } else {
                    Expr::Mul(Box::new(a), Box::new(b))
                }
            },
        )(input)
    }
}

mod part_b {
    use super::*;

    fn subexpr(input: &str) -> IResult<&str, Expr> {
        map_res(
            preceded(tag("("), terminated(expr, tag(")"))),
            |e: Expr| -> Result<Expr> { Ok(Expr::Subexpr(Box::new(e))) },
        )(input)
    }

    // Since terms have higher precedence we prioritize them
    fn terms(input: &str) -> IResult<&str, Expr> {
        let (input, init) = alt((subexpr, num))(input)?;
        fold_many0(preceded(tag(" + "), alt((subexpr, num))), init, |a, b| {
            Expr::Add(Box::new(a), Box::new(b))
        })(input)
    }

    pub fn expr<'a>(input: &'a str) -> IResult<&'a str, Expr> {
        let (input, init) = alt((terms, subexpr, num))(input)?;
        fold_many0(
            preceded(tag(" * "), alt((terms, subexpr, num))),
            init,
            |a, b| Expr::Mul(Box::new(a), Box::new(b)),
        )(input)
    }
}

fn eval_part_a(s: &str) -> Result<isize> {
    let (_, expr) = part_a::expr(s).map_err(|_| anyhow!("Invalid result"))?;
    Ok(expr.eval())
}

fn eval_part_b(s: &str) -> Result<isize> {
    let (_, expr) = part_b::expr(s).map_err(|_| anyhow!("Invalid result"))?;
    Ok(expr.eval())
}

pub fn main(path: &Path) -> Result<(isize, Option<isize>)> {
    let mut sum_a = 0;
    let mut sum_b = 0;
    for line in read_lines(path)? {
        let line = line?;
        sum_a += eval_part_a(&line)?;
        sum_b += eval_part_b(&line)?;
    }
    Ok((sum_a, Some(sum_b)))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_a_examples() -> Result<()> {
        assert_eq!(eval_part_a("1 + 2 * 3 + 4 * 5 + 6")?, 71);
        assert_eq!(eval_part_a("1 + (2 * 3) + (4 * (5 + 6))")?, 51);
        assert_eq!(eval_part_a("2 * 3 + (4 * 5)")?, 26);
        assert_eq!(eval_part_a("5 + (8 * 3 + 9 + 3 * 4 * 3)")?, 437);
        assert_eq!(
            eval_part_a("5 * 9 * (7 * 3 * 3 + 9 * 3 + (8 + 6 * 4))")?,
            12240
        );
        assert_eq!(
            eval_part_a("((2 + 4 * 9) * (6 + 9 * 8 + 6) + 6) + 2 + 4 * 2")?,
            13632
        );
        Ok(())
    }

    #[test]
    fn test_part_b_examples() -> Result<()> {
        assert_eq!(eval_part_b("1 + 2 * 3 + 4 * 5 + 6")?, 231);
        assert_eq!(eval_part_b("1 + (2 * 3) + (4 * (5 + 6))")?, 51);
        assert_eq!(eval_part_b("2 * 3 + (4 * 5)")?, 46);
        assert_eq!(eval_part_b("5 + (8 * 3 + 9 + 3 * 4 * 3)")?, 1445);
        assert_eq!(
            eval_part_b("5 * 9 * (7 * 3 * 3 + 9 * 3 + (8 + 6 * 4))")?,
            669060
        );
        assert_eq!(
            eval_part_b("((2 + 4 * 9) * (6 + 9 * 8 + 6) + 6) + 2 + 4 * 2")?,
            23340
        );
        Ok(())
    }
}
