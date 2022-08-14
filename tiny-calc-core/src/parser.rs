// use nom::branch::alt;
// use nom::character::complete::{char, i64, space0};
// use nom::combinator::{eof, map};
// use nom::sequence::tuple;
// use nom::IResult;

use nom::branch::alt;
use nom::character::complete::{char, digit1, space0};
use nom::combinator::map;
use nom::multi::many0;
use nom::sequence::{delimited, tuple};
use nom::IResult;
// use std::str::FromStr;

use crate::ast::*;

pub fn parse(input: &str) -> Result<Calculation, nom::Err<nom::error::Error<&str>>> {
    // TODO: parse fully
    parse_math_expr(input).map(|(_rem, expr)| Calculation {
        expr,
        formatter: None,
    })
}

// fn parse_basic_expr(input: &str) -> IResult<&str, Calculation> {
//     parse_math_expr(input).map(|expr| Calculation {
//         expr,
//         formatter: None,
//     })
// }

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

fn parse_op(tup: (char, Expr), expr1: Expr) -> Expr {
    let (op, expr2) = tup;
    // match op {
    //     '+' => EAdd(Box::new(expr1), Box::new(expr2)),
    //     '-' => ESub(Box::new(expr1), Box::new(expr2)),
    //     '*' => EMul(Box::new(expr1), Box::new(expr2)),
    //     '/' => EDiv(Box::new(expr1), Box::new(expr2)),
    //     '^' => EExp(Box::new(expr1), Box::new(expr2)),
    //     _ => panic!("Unknown Operation"),
    // }

    let op = match op {
        '+' => Op::Add,
        '-' => Op::Sub,
        '*' => Op::Mul,
        _ => panic!("unknown operator"),
    };

    Expr::BinaryOp(BinaryOp {
        lhs: Box::new(expr1),
        op,
        rhs: Box::new(expr2),
    })
}

fn parse_enum(parsed_num: &str) -> Expr {
    let num = i64::from_str_radix(parsed_num, 10).unwrap();
    Expr::Integer(Integer { value: num })
}

fn parse_number(input: &str) -> IResult<&str, Expr> {
    map(delimited(space0, digit1, space0), parse_enum)(input)
}

// fn integer(input: &str) -> IResult<&str, Expr> {
//     let (input, value) = i64(input)?;
//     Ok((input, Expr::Integer(Integer { value })))
// }

// fn term(input: &str) -> IResult<&str, Expr> {
//     alt((
//         map(
//             tuple((integer, space0, alt((char('+'), char('-'))), space0, term)),
//             // tuple((integer, space0, char('*'), space0, term)),
//             |(lhs, _, op, _, rhs)| {
//                 let op = match op {
//                     '+' => Op::Add,
//                     '-' => Op::Sub,
//                     '*' => Op::Mul,
//                     _ => unreachable!(),
//                 };

//                 Expr::BinaryOp(BinaryOp {
//                     lhs: Box::new(lhs),
//                     op,
//                     rhs: Box::new(rhs),
//                 })
//             },
//         ),
//         map(integer, |i| i),
//     ))(input)
// }

// fn expr(input: &str) -> IResult<&str, Expr> {
//     alt((
//         map(
//             // tuple((term, space0, alt((char('+'), char('-'))), space0, expr)),
//             tuple((term, space0, char('*'), space0, expr)),
//             |(lhs, _, op, _, rhs)| {
//                 let op = match op {
//                     '+' => Op::Add,
//                     '-' => Op::Sub,
//                     '*' => Op::Mul,
//                     _ => unreachable!(),
//                 };

//                 Expr::BinaryOp(BinaryOp {
//                     lhs: Box::new(lhs),
//                     op,
//                     rhs: Box::new(rhs),
//                 })
//             },
//         ),
//         // map(
//         //     tuple((term, space0, char('-'), space0, expr)),
//         //     |(lhs, _, _, _, rhs)| {
//         //         Expr::BinaryOp(BinaryOp {
//         //             lhs: Box::new(lhs),
//         //             op: Op::Sub,
//         //             rhs: Box::new(rhs),
//         //         })
//         //     },
//         // ),
//         map(term, |t| t),
//     ))(input)
// }

// fn fully<A>(parser: impl FnMut(&str) -> IResult<&str, A>, input: &str) -> IResult<&str, A> {
//     map(tuple((space0, parser, eof)), |(_, expr, _)| expr)(input)
// }

// pub fn parse(input: &str) -> Result<Calculation, nom::Err<nom::error::Error<&str>>> {
//     fully(expr, input).map(|(_, expr)| Calculation {
//         expr,
//         formatter: None,
//     })
// }

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
