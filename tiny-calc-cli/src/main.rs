use std::io::BufRead;

use tiny_calc_core::{evaluator, parser};

fn main() {
    let stdin = std::io::stdin();

    for line in stdin.lock().lines() {
        let line = line.unwrap();
        let parse_result = parser::parse(&line);

        match parse_result {
            Ok(calc) => {
                let output = evaluator::eval(calc);
                println!("{output}");
            },
            Err(_) => println!(" Syntax error"),
        }
    }
}
