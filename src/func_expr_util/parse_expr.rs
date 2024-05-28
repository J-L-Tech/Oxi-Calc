use super::expr_types::*;

/**
 * 
 */
pub fn parse_expr(expr_str: &str) -> Result<Expr, ParseError> {
    if let Ok(int) = expr_str.trim().parse::<i64>() {
        return Ok(Expr::Integer { i: int });
    }
    else if let Ok(float) = expr_str.trim().parse::<f64>() {
        return Ok(Expr::Float { f: float });
    }
    else if let Ok(func) = parse_fn(expr_str) {
        return Ok(func);
    }
    else {
        return Err(format!("Expression String Does not match any pattern in grammar {}", expr_str).into());
    }
}

fn identifier_to_builtin(identifier: &str) -> Result<Builtin, ParseError> {
    match identifier {
        "abs" =>            Ok(Builtin::Abs),
        "gcd" =>            Ok(Builtin::GCD),
        "choose" =>         Ok(Builtin::Choose),
        "permutation" =>    Ok(Builtin::Permutation),
        "round" =>          Ok(Builtin::Round),
        "truncate" =>       Ok(Builtin::Truncate),
        "fraction" =>       Ok(Builtin::Fraction),
        "sin" =>            Ok(Builtin::Sin),
        "cos" =>            Ok(Builtin::Cos),
        "tan" =>            Ok(Builtin::Tan),
        "arcsin" =>         Ok(Builtin::ArcSin),
        "arcos" =>          Ok(Builtin::ArcCos),
        "arctan" =>         Ok(Builtin::ArcTan),
        "sinh" =>           Ok(Builtin::Sinh),
        "cosh" =>           Ok(Builtin::Cosh),
        "tanh" =>           Ok(Builtin::Tanh),
        "arcsinh" =>        Ok(Builtin::ArcSinh),
        "arccosh" =>        Ok(Builtin::ArcCosh),
        "arctanh" =>        Ok(Builtin::ArcTanh),
        "ln" =>             Ok(Builtin::Ln),
        "log_10" =>         Ok(Builtin::Log10),
        "log_2" =>          Ok(Builtin::Log2),
        "pow" =>            Ok(Builtin::Pow),
        "max" =>            Ok(Builtin::Max),
        "min" =>            Ok(Builtin::Min),
        "rand" =>           Ok(Builtin::Rand),
        "randint" =>        Ok(Builtin::RandInt),
        "percent_of" =>     Ok(Builtin::PercentOf),
        "add" | "+" =>      Ok(Builtin::Add),
        "sub" | "-" =>      Ok(Builtin::Subtract),
        "mult"  | "*" =>    Ok(Builtin::Multiply),
        "div" | "/" =>      Ok(Builtin::Divide),
        "mod" | "%" =>      Ok(Builtin::Modulus),
        _ =>                Err("No Matching Builtin Function".into()),
    }
}

fn args_to_vec(args: &str) -> Result<Vec<Expr>, ParseError> {
    let mut arg_vec: Vec<Expr> = Vec::new();
    let mut delim_vec: Vec<char> = Vec::new();
    let mut current_arg: String = String::new();
    for (i, c) in args.chars().enumerate() {
        if i == 0 {                   // First Character
            continue;
        }
        else if i == args.len() - 1 { // Last Character
            if delim_vec.is_empty() {
                if c != ')' {
                    return Err("Function Parameters are not closed".into());
                }
                else if current_arg.is_empty() {
                    continue;
                }
                else {
                    let expr: Expr = parse_expr(&current_arg)?;
                    arg_vec.push(expr);
                    current_arg.clear();
                }
            }
            else {
                return Err("Delimiter Mismatch".into());
            }
        }
        else {
            match c {
                '(' => {
                    delim_vec.push(c);
                    current_arg.push(c);
                },
                ')' => {
                    if delim_vec.is_empty() {
                        return Err("Delimiter Mismatch".into()); 
                    }
                    else {
                        delim_vec.pop();
                        current_arg.push(c);
                    }
                },
                ',' => {
                    if delim_vec.len() == 0 {
                        let expr: Expr = parse_expr(&current_arg)?;
                        arg_vec.push(expr);
                        current_arg.clear();
                    }
                    else {
                        current_arg.push(c);
                    }
                }
                ' ' | '\n' | '\t' => {
                    continue;
                }
                _ => {
                    current_arg.push(c);
                }
            }
        }
    }

    return Ok(arg_vec);
}

fn parse_fn(in_string: &str) -> Result<Expr, ParseError> {
    if let Some(pos) = in_string.find('(') {
        return match in_string.split_at(pos) {
            (identifier, args) => {
                Ok(Expr::BuiltinFn { name: identifier_to_builtin(identifier)?, args: args_to_vec(args)? })
            }
        };
    }
    else {
        return Err("No Matching Builtin Function".into());
    }
}

#[cfg(test)]
mod parse_tests {
    use super::*;
    use Expr::*;

    #[test]
    fn function_parsing_0_arg() {
        assert_eq!(
            Ok(BuiltinFn { name: Builtin::Rand, args: vec![] }),
            parse_expr("rand()")
        );
    }

    #[test]
    fn function_parsing_1_arg() {
        assert_eq!(
            Ok(BuiltinFn { name: Builtin::Abs, args: (vec![Integer { i: -1 }]) }),
            parse_expr("abs(-1)")
        );
    }

    #[test]
    fn function_parsing_2_arg() {
        assert_eq!(
            Ok(BuiltinFn { name: Builtin::GCD, args: (vec![Integer { i: 12 }, Integer { i: 4 }]) }),
            parse_expr("gcd(12, 4)")
        );
    }

    #[test]
    fn function_parsing_func_arg() {
        assert_eq!(
            Ok(BuiltinFn { name: Builtin::Add, args: (vec![Integer { i: 2 }, BuiltinFn {name: Builtin::Add, args: vec![ Integer { i: 2 }, Integer{i: 2}]}]) }),
            parse_expr("add(2, add(2,2))")
        );
    }

    #[test]
    fn int_parsing() {
        assert_eq!(
            Ok(Integer { i: 1 }),
            parse_expr("1")
        );
    }

    #[test]
    fn float_parsing() {
        assert_eq!(
            Ok(Float { f: 1.0 }),
            parse_expr("1.0")
        );
    }

    #[test]
    fn negate_int_parsing() {
        assert_eq!(
            Ok(Integer { i: -1 }),
            parse_expr("-1")
        );
    }

    #[test]
    fn negate_float_parsing() {
        assert_eq!(
            Ok(Float { f: -1.0 }),
            parse_expr("-1.0")
        );
    }
}