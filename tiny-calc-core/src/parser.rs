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

fn term(input: &str) -> IResult<&str, Expr> {
    alt((
        map(
            tuple((integer, space0, char('*'), space0, term)),
            |(lhs, _, _, _, rhs)| {
                Expr::BinaryOp(BinaryOp {
                    lhs: Box::new(Expr::Integer(lhs)),
                    op: Op::Mul,
                    rhs: Box::new(rhs),
                })
            },
        ),
        map(integer, |i| Expr::Integer(i)),
    ))(input)
}

fn expr(input: &str) -> IResult<&str, Expr> {
    alt((
        map(
            tuple((term, space0, alt((char('+'), char('-'))), space0, expr)),
            |(lhs, _, op, _, rhs)| {
                let op = match op {
                    '+' => Op::Add,
                    '-' => Op::Sub,
                    _ => unreachable!(),
                };

                Expr::BinaryOp(BinaryOp {
                    lhs: Box::new(lhs),
                    op,
                    rhs: Box::new(rhs),
                })
            },
        ),
        // map(
        //     tuple((term, space0, char('-'), space0, expr)),
        //     |(lhs, _, _, _, rhs)| {
        //         Expr::BinaryOp(BinaryOp {
        //             lhs: Box::new(lhs),
        //             op: Op::Sub,
        //             rhs: Box::new(rhs),
        //         })
        //     },
        // ),
        map(term, |t| t),
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

        // TODO: this AST is wrong. Wrong precedence.
        assert_eq!(
            output,
            Calculation {
                expr: Expr::BinaryOp(BinaryOp {
                    lhs: Box::new(Expr::Integer(Integer { value: 10 })),
                    rhs: Box::new(Expr::BinaryOp(BinaryOp {
                        lhs: Box::new(Expr::BinaryOp(BinaryOp {
                            lhs: Box::new(Expr::Integer(Integer { value: 2 })),
                            rhs: Box::new(Expr::Integer(Integer { value: 3 })),
                            op: Op::Mul,
                        })),
                        rhs: Box::new(Expr::Integer(Integer { value: 1 })),
                        op: Op::Add,
                    })),
                    op: Op::Sub,
                }),
                formatter: None
            }
        );
    }
}
