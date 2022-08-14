use nom::branch::alt;
use nom::character::complete::{char, i64, space0};
use nom::combinator::{eof, map};
use nom::sequence::tuple;
use nom::IResult;

use crate::ast::*;

fn integer(input: &str) -> IResult<&str, Integer> {
    let (input, value) = i64(input)?;
    Ok((input, Integer { value }))
}

pub fn expr(input: &str) -> IResult<&str, Expr> {
    alt((
        map(
            tuple((integer, space0, char('+'), space0, expr)),
            |(lhs, _, _, _, rhs)| {
                Expr::BinaryOp(BinaryOp {
                    lhs: Box::new(Expr::Integer(lhs)),
                    op: Op::Add,
                    rhs: Box::new(rhs),
                })
            },
        ),
        map(
            tuple((integer, space0, char('-'), space0, expr)),
            |(lhs, _, _, _, rhs)| {
                Expr::BinaryOp(BinaryOp {
                    lhs: Box::new(Expr::Integer(lhs)),
                    op: Op::Sub,
                    rhs: Box::new(rhs),
                })
            },
        ),
        map(integer, |i| Expr::Integer(i)),
    ))(input)
}

fn fully<A>(parser: impl FnMut(&str) -> IResult<&str, A>, input: &str) -> IResult<&str, A> {
    map(tuple((space0, parser, eof)), |(_, expr, _)| expr)(input)
}

pub fn parse(input: &str) -> Result<Calculation, nom::Err<nom::error::Error<&str>>> {
    fully(expr, input).map(|(_, expr)| Calculation {
        expr,
        formatter: None,
    })
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
    }
}
