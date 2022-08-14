use nom::branch::alt;
use nom::character::complete::{char, i64, space0};
use nom::combinator::{eof, map};
use nom::multi::many0;
use nom::sequence::{delimited, tuple};
use nom::IResult;

use crate::ast::*;

pub fn parse(input: &str) -> Result<Calculation, nom::Err<nom::error::Error<&str>>> {
    fully(parse_math_expr, input).map(|(_rem, expr)| Calculation {
        expr,
        formatter: None,
    })
}

fn parse_parens(input: &str) -> IResult<&str, Expr> {
    delimited(
        space0,
        delimited(char('('), parse_math_expr, char(')')),
        space0,
    )(input)
}

fn parse_operation(input: &str) -> IResult<&str, Expr> {
    alt((parse_parens, parse_number))(input)
}

fn parse_factor(input: &str) -> IResult<&str, Expr> {
    let (input, num1) = parse_operation(input)?;
    let (input, exprs) = many0(tuple((char('^'), parse_factor)))(input)?;
    Ok((input, parse_expr(num1, exprs)))
}

fn parse_term(input: &str) -> IResult<&str, Expr> {
    let (input, num1) = parse_factor(input)?;
    let (input, exprs) = many0(tuple((alt((char('/'), char('*'))), parse_factor)))(input)?;
    Ok((input, parse_expr(num1, exprs)))
}

fn parse_math_expr(input: &str) -> IResult<&str, Expr> {
    let (input, num1) = parse_term(input)?;
    let (input, exprs) = many0(tuple((alt((char('+'), char('-'))), parse_term)))(input)?;
    Ok((input, parse_expr(num1, exprs)))
}

fn parse_expr(expr: Expr, rem: Vec<(char, Expr)>) -> Expr {
    rem.into_iter().fold(expr, |acc, val| parse_op(val, acc))
}

fn parse_op((op, rhs): (char, Expr), lhs: Expr) -> Expr {
    let op = match op {
        // TODO: div and exp
        '+' => Op::Add,
        '-' => Op::Sub,
        '*' => Op::Mul,
        _ => panic!("unknown operator"),
    };

    Expr::BinaryOp(BinaryOp {
        lhs: Box::new(lhs),
        op,
        rhs: Box::new(rhs),
    })
}

fn parse_number(input: &str) -> IResult<&str, Expr> {
    map(delimited(space0, i64, space0), |value| {
        Expr::Integer(Integer { value })
    })(input)
}

fn fully<A>(parser: impl FnMut(&str) -> IResult<&str, A>, input: &str) -> IResult<&str, A> {
    map(tuple((space0, parser, eof)), |(_, expr, _)| expr)(input)
}
#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn parse_works() {
        let output = parse("1").unwrap();
        assert_eq!(
            output,
            Calculation {
                expr: Expr::Integer(Integer { value: 1i64 }),
                formatter: None
            }
        );

        let output = parse("2 + 3").unwrap();
        assert_eq!(
            output,
            Calculation {
                expr: Expr::BinaryOp(BinaryOp {
                    lhs: Box::new(Expr::Integer(Integer { value: 2 })),
                    op: Op::Add,
                    rhs: Box::new(Expr::Integer(Integer { value: 3 })),
                }),
                formatter: None
            }
        );

        let output = parse("2 - 3").unwrap();
        assert_eq!(
            output,
            Calculation {
                expr: Expr::BinaryOp(BinaryOp {
                    lhs: Box::new(Expr::Integer(Integer { value: 2 })),
                    op: Op::Sub,
                    rhs: Box::new(Expr::Integer(Integer { value: 3 })),
                }),
                formatter: None
            }
        );

        let output = parse("2 * 3").unwrap();
        assert_eq!(
            output,
            Calculation {
                expr: Expr::BinaryOp(BinaryOp {
                    lhs: Box::new(Expr::Integer(Integer { value: 2 })),
                    op: Op::Mul,
                    rhs: Box::new(Expr::Integer(Integer { value: 3 })),
                }),
                formatter: None
            }
        );

        let output = parse("2 - 1 * 3").unwrap();
        assert_eq!(
            output,
            Calculation {
                expr: Expr::BinaryOp(BinaryOp {
                    lhs: Box::new(Expr::Integer(Integer { value: 2 })),
                    rhs: Box::new(Expr::BinaryOp(BinaryOp {
                        lhs: Box::new(Expr::Integer(Integer { value: 1 })),
                        rhs: Box::new(Expr::Integer(Integer { value: 3 })),
                        op: Op::Mul
                    })),
                    op: Op::Sub
                }),
                formatter: None
            }
        );

        let output = parse("10 - 2 * 3 + 1").unwrap();

        assert_eq!(
            output,
            Calculation {
                expr: Expr::BinaryOp(BinaryOp {
                    lhs: Box::new(Expr::BinaryOp(BinaryOp {
                        lhs: Box::new(Expr::Integer(Integer { value: 10 })),
                        rhs: Box::new(Expr::BinaryOp(BinaryOp {
                            lhs: Box::new(Expr::Integer(Integer { value: 2 })),
                            rhs: Box::new(Expr::Integer(Integer { value: 3 })),
                            op: Op::Mul
                        })),
                        op: Op::Sub
                    })),
                    rhs: Box::new(Expr::Integer(Integer { value: 1 })),
                    op: Op::Add
                }),
                formatter: None
            }
        );
    }
}
