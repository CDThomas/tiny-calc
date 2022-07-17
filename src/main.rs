#[macro_use]
extern crate lazy_static;
extern crate pest;
#[macro_use]
extern crate pest_derive;

use pest::iterators::{Pair, Pairs};
use pest::prec_climber::*;
use pest::Parser;
use std::io::BufRead;

#[derive(Parser)]
#[grammar = "grammar.pest"]
struct Calculator;

lazy_static! {
    static ref PREC_CLIMBER: PrecClimber<Rule> = {
        use Assoc::*;
        use Rule::*;

        PrecClimber::new(vec![
            Operator::new(add, Left) | Operator::new(subtract, Left),
            Operator::new(multiply, Left) | Operator::new(divide, Left),
            Operator::new(power, Right),
        ])
    };
}

fn eval(expression: Pairs<Rule>) -> i64 {
    PREC_CLIMBER.climb(
        expression,
        |pair: Pair<Rule>| match pair.as_rule() {
            // TODO: this can overflow so it'd be nice to print a more informative error
            Rule::int => pair.as_str().parse::<i64>().unwrap(),
            Rule::binary_literal => {
                // Unwrap is fine here because parsing has already succeeded
                let without_prefix = pair.as_str().strip_prefix("0b").unwrap();
                // TODO: This can still overflow, so it'd be nice to print a more informative error
                i64::from_str_radix(without_prefix, 2).unwrap()
            }
            Rule::hex_leteral => {
                // Unwrap is fine here because parsing has already succeeded
                let without_prefix = pair.as_str().strip_prefix("0x").unwrap();
                // TODO: This can still overflow, so it'd be nice to print a more informative error
                i64::from_str_radix(without_prefix, 16).unwrap()
            }
            Rule::expr => eval(pair.into_inner()),
            Rule::calculation => {
                let mut inner = pair.into_inner();
                let result = eval(Pairs::single(inner.next().unwrap()));

                let formatter = inner.next().unwrap().as_str();

                match formatter {
                    "#x" => println!(" = {:#X}", result),
                    "#b" => println!(" = {:#b}", result),
                    _ => println!(" = {}", result)
                }

                result
            }
            _ => unreachable!(),
        },
        |lhs: i64, op: Pair<Rule>, rhs: i64| match op.as_rule() {
            Rule::add => lhs + rhs,
            Rule::subtract => lhs - rhs,
            Rule::multiply => lhs * rhs,
            Rule::divide => lhs / rhs,
            Rule::power => lhs.pow(rhs.try_into().unwrap()),
            _ => unreachable!(),
        },
    )
}

fn main() {
    let stdin = std::io::stdin();

    for line in stdin.lock().lines() {
        let line = line.unwrap();
        let parse_result = Calculator::parse(Rule::calculation, &line);

        match parse_result {
            Ok(calc) => {
                eval(calc);
            },
            Err(_) => println!(" Syntax error"),
        }
    }
}
