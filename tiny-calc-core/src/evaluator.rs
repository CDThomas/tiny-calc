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
        let ast = parse("1").unwrap();
        let output = eval(ast);

        assert_eq!(output, " = 1");

        let ast = parse("1 + 2").unwrap();
        let output = eval(ast);

        assert_eq!(output, " = 3");

        let ast = parse("1 + 2 * 3").unwrap();
        let output = eval(ast);

        assert_eq!(output, " = 7");

        let ast = parse("10 - 2 * 3").unwrap();
        let output = eval(ast);

        assert_eq!(output, " = 4");

        let ast = parse("10 - 2 * 3 + 1").unwrap();
        let output = eval(ast);

        assert_eq!(output, " = 5");
    }
}
