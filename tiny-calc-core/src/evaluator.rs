use crate::ast::*;

pub fn eval(calc: Calculation) -> String {
    let output = eval_expr(calc.expr);
    format!(" = {output}")
}

pub fn eval_expr(expr: Expr) -> i64 {
    match expr {
        Expr::Integer(int) => int.value,
        Expr::BinaryOp(BinaryOp{ lhs, rhs, op }) => {
            match op {
                Op::Add => eval_expr(*lhs) + eval_expr(*rhs),
                Op::Sub => eval_expr(*lhs) - eval_expr(*rhs),
                Op::Mul => eval_expr(*lhs) * eval_expr(*rhs),
            }
        },
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::parser::parse;

    #[test]
    fn eval_works() {
        let cases = vec![
            ("1", " = 1"),
            ("1 + 2", " = 3"),
            ("1 + 2 * 3", " = 7"),
            ("10 - 2 * 3", " = 4"),
            ("10 - 2 * 3 + 1", " = 5"),
        ];

        for (input, expected) in cases {
            let ast = parse(input).expect("failed to parse");
            assert_eq!(eval(ast), expected);
        }
    }
}
