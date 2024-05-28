use rand::{random, thread_rng, Rng};

use super::expr_types::*;

pub fn interp_expr(expr: &Expr) -> Value {
    let mut rng: rand::prelude::ThreadRng = thread_rng();

    use Expr::*;
    match expr {
        Integer {i} => Value::IntV {i_v: *i},
        Float {f} => Value::FloatV {f_v: *f},
        BuiltinFn { name, args } => {
            match name {
                Builtin::Sin =>     do_single_arg_f64(f64::sin, &args),
                Builtin::Cos =>     do_single_arg_f64(f64::cos, &args),
                Builtin::Tan =>     do_single_arg_f64(f64::tan, &args),
                Builtin::ArcSin =>  do_single_arg_f64(f64::asin, &args),
                Builtin::ArcCos =>  do_single_arg_f64(f64::acos, &args),
                Builtin::ArcTan =>  do_single_arg_f64(f64::atan, &args),
                Builtin::Ln =>      do_single_arg_f64(f64::ln, &args),
                Builtin::Log10 =>   do_single_arg_f64(f64::log10, &args),
                Builtin::Log2 =>    do_single_arg_f64(f64::log2, &args),
                Builtin::Sinh =>    do_single_arg_f64(f64::sinh, &args),
                Builtin::Cosh =>    do_single_arg_f64(f64::cosh, &args),
                Builtin::Tanh =>    do_single_arg_f64(f64::tanh, &args),
                Builtin::ArcSinh => do_single_arg_f64(f64::asinh, &args),
                Builtin::ArcCosh => do_single_arg_f64(f64::acosh, &args),
                Builtin::ArcTanh => do_single_arg_f64(f64::atanh, &args),
                Builtin::Pow => {
                    if args.len() != 2 {
                       return Value::Error{msg: "Incorrect Number of Args".to_string()};
                    }
                    let a_v: Value = interp_expr(&args[0]);
                    let b_v: Value = interp_expr(&args[1]);
                    match pair_up(a_v, b_v) {
                        ValuePair::IntInt { a, b } => {
                            if b < 0 {
                                return Value::Error{msg: "Cannot Exponentiate by Negative Number".to_string()};
                            }
                            Value::IntV { i_v: a.pow(b as u32) }
                        },
                        ValuePair::FloatInt { a, b } => {
                            match b.try_into() {
                                Ok(b_pow) => {
                                    return Value::FloatV { f_v: a.powi(b_pow) }
                                },
                                Err(b) => {
                                    Value::Error { msg: b.to_string() }
                                }

                            }
                        },
                        ValuePair::IntFloat { a, b } => Value::FloatV { f_v: (a as f64).powf(b) },
                        ValuePair::FloatFloat { a, b } => Value::FloatV { f_v: a.powf(b) },
                    }
                },
                Builtin::Abs => {
                    if args.len() != 1 {
                        return Value::Error{msg: "Incorrect Number of Args".to_string()};
                    }
                    let a_v: Value = interp_expr(&args[0]);
                    match a_v {
                        Value::IntV { i_v } => Value::IntV { i_v: i_v.abs() },
                        Value::FloatV { f_v } => Value::FloatV { f_v: f_v.abs() },
                        Value::Error { msg } => Value::Error { msg: msg },
                    }
                },
                Builtin::Choose => {
                    if args.len() != 2 {
                        return Value::Error{msg: "Incorrect Number of Args".to_string()};
                    }
                    let a_v: Value = interp_expr(&args[0]);
                    let b_v: Value = interp_expr(&args[1]);
                    match pair_up(a_v, b_v) {
                        ValuePair::IntInt { a, b } => Value::IntV {i_v: factorial(a) / (factorial(b) * factorial(a - b))},
                        _ => Value::Error { msg: "Int Parameters Only".to_string() }
                    } 
                },
                Builtin::Permutation => {
                    let a_v: Value = interp_expr(&args[0]);
                    let b_v: Value = interp_expr(&args[1]);
                    match pair_up(a_v, b_v) {
                        ValuePair::IntInt { a, b } => {
                            if a >= 0 && b >= 0 && b <= u32::MAX as i64 {
                                Value::IntV {i_v: a.pow(b as u32)}
                            }
                            else {
                                Value::Error { msg: " Positive Integer Parameters Only".to_string() }
                            }
                        },
                        _ => Value::Error { msg: "Int Parameters Only".to_string() }
                    } 
                },
                Builtin::Round => {
                    if args.len() != 1 {
                        return Value::Error{msg: "Incorrect Number of Args".to_string()};
                    }
                    let a_v: Value = interp_expr(&args[0]);
                    match a_v {
                        Value::IntV { i_v } => Value::IntV { i_v: i_v },
                        Value::FloatV { f_v } => Value::FloatV { f_v: f_v.round() },
                        Value::Error { msg } => Value::Error { msg: msg },
                    }
                },
                Builtin::Truncate => {
                    if args.len() != 1 {
                        return Value::Error{msg: "Incorrect Number of Args".to_string()};
                    }
                    let a_v: Value = interp_expr(&args[0]);
                    match a_v {
                        Value::IntV { i_v } => Value::IntV { i_v: i_v },
                        Value::FloatV { f_v } => Value::FloatV { f_v: f_v.trunc() },
                        Value::Error { msg } => Value::Error { msg: msg },
                    }
                }
                Builtin::Fraction => {
                    if args.len() != 1 {
                        return Value::Error{msg: "Incorrect Number of Args".to_string()};
                    }
                    let a_v: Value = interp_expr(&args[0]);
                    match a_v {
                        Value::IntV { i_v } => Value::FloatV { f_v: 0.0 },
                        Value::FloatV { f_v } => Value::FloatV { f_v: f_v.fract() },
                        Value::Error { msg } => Value::Error { msg: msg },
                    }
                },
                Builtin::Max => {
                    do_binop(&args, |l, r| l.max(r), |l, r| l.max(r))
                },
                Builtin::Min => {
                    do_binop(&args, |l, r| l.min(r), |l, r| l.min(r))
                },
                Builtin::Rand => {
                    if !args.is_empty() {
                        return Value::Error{msg: "Incorrect Number of Args".to_string()};
                    }
                    Value::FloatV {f_v: rng.gen()}
                },
                Builtin::RandInt => {
                    match args.len() {
                        0 => Value::IntV{ i_v: random::<i64>()},
                        1 => {
                            let a_v: Value = interp_expr(&args[0]);
                            match a_v {
                                Value::IntV { i_v } => Value::IntV { i_v: rng.gen_range(0..i_v) },
                                Value::FloatV { f_v } => Value::Error { msg: "Float in Int Parameter".to_string() },
                                Value::Error { msg } => Value::Error { msg: msg },
                            } 
                        },
                        2 => {
                            let a_v: Value = interp_expr(&args[0]);
                            let b_v: Value = interp_expr(&args[1]);
                            match pair_up(a_v, b_v) {
                                ValuePair::IntInt { a, b } => Value::IntV {i_v: rng.gen_range(a..b)},
                                _ => Value::Error { msg: "Int Parameters Only".to_string() }
                            } 
                        }
                        _ => Value::Error { msg: "Invalid Number of Args".to_string()}
                    }
                },
                Builtin::GCD => {
                    if args.len() != 2 {
                        return Value::Error{ msg: "Invalid Number of Args".to_string()}
                    }
                    let l_v: Value = interp_expr(&args[0]);
                    let r_v: Value = interp_expr(&args[1]);
                    match pair_up(l_v, r_v) {
                        ValuePair::IntInt { a, b } => {
                            Value::IntV{ i_v: gcd(a, b) }
                        },
                        _ => Value::Error { msg: "Integer Arguments Only".to_string() }
                    }
                },
                Builtin::PercentOf => {
                    if args.len() != 2 {
                        return Value::Error{ msg: "Invalid Number of Args".to_string()}
                    }
                    let l_v: Value = interp_expr(&args[0]);
                    let r_v: Value = interp_expr(&args[1]);
                    match pair_up(l_v, r_v) {
                        ValuePair::IntInt { a, b } => {
                            Value::FloatV{ f_v: a as f64 * (b as f64 / 100.0) }
                        },
                        ValuePair::IntFloat { a, b } => {
                            Value::FloatV{ f_v: a as f64 * (b as f64 / 100.0) }
                        },
                        ValuePair::FloatInt { a, b } => {
                            Value::FloatV{ f_v: a as f64 * (b as f64 / 100.0) }
                        },
                        ValuePair::FloatFloat { a, b } => {
                            Value::FloatV{ f_v: a as f64 * (b as f64 / 100.0) }
                        },
                    }
                },
                Builtin::Add => do_binop(&args, |a, b| {a + b}, |a, b| {a + b}),
                Builtin::Subtract => do_binop(&args, |a, b| {a - b}, |a, b| {a - b}),
                Builtin::Multiply => do_binop(&args, |a, b| {a * b}, |a, b| {a * b}),
                Builtin::Divide => do_binop(&args, |a, b| {a / b}, |a, b| {a / b}),
                Builtin::Modulus => do_binop(&args, |a, b| {a % b}, |a, b| {a % b}),
            }
        },
    }
}

fn do_binop(args: &Vec<Expr>, int_op: fn(i64, i64) -> i64, float_op: fn(f64, f64) -> f64) -> Value {
    if args.len() != 2 {
        return Value::Error{ msg: "Invalid Number of Args".to_string()}
    }
    let a_v: Value = interp_expr(&args[0]);
    let b_v: Value = interp_expr(&args[1]);
    match pair_up(a_v, b_v) {
        ValuePair::IntInt { a, b } => Value::IntV { i_v: int_op(a,  b) },
        ValuePair::FloatInt { a, b } => Value::FloatV { f_v: float_op(a, b as f64) },
        ValuePair::IntFloat { a, b } => Value::FloatV { f_v: float_op(a as f64, b) },
        ValuePair::FloatFloat { a, b } => Value::FloatV { f_v: float_op(a, b) },
    }
}

fn do_single_arg_f64(single_arg_fn: fn(f64) -> f64, args: &Vec<Expr>) -> Value {
    if args.len() != 1 {
        return Value::Error {msg: format!("Incorrect Length {}", args.len())}
    }
    let a_v: Value = interp_expr(&args[0]);
    match a_v {
        Value::IntV { i_v } => Value::FloatV{f_v: single_arg_fn(i_v as f64)},
        Value::FloatV { f_v } => Value::FloatV{f_v: single_arg_fn(f_v)},
        Value::Error { msg } => Value::Error { msg: msg },
    }
}

fn factorial_value(n_v : Value) -> Value {
    match n_v {
        Value::IntV { i_v } => {
            if i_v < 0 {
                Value::Error{msg: "Cannot Apply Factorial to Negative Integers".to_string()}
            }
            else {
                Value::IntV { i_v: factorial(i_v) }
            }
        },
        Value::FloatV { f_v } => Value::Error{msg: "Cannot Apply Factorial to f64".to_string()},
        Value::Error { msg } => Value::Error{msg: msg},
    }
}

fn factorial(i: i64) -> i64 {
    return (1..=i).fold(1, |acc: i64, i: i64 | acc * i);
}

fn gcd(a: i64, b: i64) -> i64 {
    if a == 0 {
        return b;
    }
    return gcd(b % a, a);
}

#[cfg(test)]
mod interp_tests {
    use super::*;
    use Expr::*;

    // macro_rules! interp_expr_eq_test {
    //     ($test_name : ident, $key : expr, $raw_expr : expr ) => {
    //         #[test]
    //         fn $test_name() {
    //             assert_eq!($key, interp_expr(&$raw_expr));
    //         }
    //     };
    // }

    // interp_expr_eq_test!(
    //     float_add, 
    //     Value::FloatV { f_v: 4.0 },
    //     BuiltinFn { 
    //         name: Builtin::Add,
    //         args: vec![Float { f: 2.0 }, Float { f: 2.0 }]
    //     }
    // );

    #[test]
    fn basic_arithmetic_builtins() {
        assert_eq!(
            Value::FloatV { f_v: 4.0 }, 
            interp_expr(&BuiltinFn { 
                name: Builtin::Add,
                args: vec![Float { f: 2.0 }, Float { f: 2.0 }]
            })
        );

        assert_eq!(
            Value::IntV { i_v: 4 }, 
            interp_expr(&BuiltinFn { 
                name: Builtin::Add,
                args: vec![Integer { i: 2 }, Integer { i: 2 }]
            })
        );

        assert_eq!(
            Value::FloatV { f_v: 0.0 }, 
            interp_expr(&BuiltinFn { 
                name: Builtin::Subtract,
                args: vec![Float { f: 2.0 }, Float { f: 2.0 }]
            })
        );

        assert_eq!(
            Value::IntV { i_v: 0 }, 
            interp_expr(&BuiltinFn { 
                name: Builtin::Subtract,
                args: vec![Integer { i: 2 }, Integer { i: 2 }]
            })
        );

        assert_eq!(
            Value::FloatV { f_v: 4.0 }, 
            interp_expr(&BuiltinFn { 
                name: Builtin::Multiply,
                args: vec![Float { f: 2.0 }, Float { f: 2.0 }]
            })
        );

        assert_eq!(
            Value::IntV { i_v: 4 }, 
            interp_expr(&BuiltinFn { 
                name: Builtin::Multiply,
                args: vec![Integer { i: 2 }, Integer { i: 2 }]
            })
        );

        assert_eq!(
            Value::FloatV { f_v: 1.0 }, 
            interp_expr(&BuiltinFn { 
                name: Builtin::Divide,
                args: vec![Float { f: 2.0 }, Float { f: 2.0 }]
            })
        );

        assert_eq!(
            Value::IntV { i_v: 1 }, 
            interp_expr(&BuiltinFn { 
                name: Builtin::Divide,
                args: vec![Integer { i: 2 }, Integer { i: 2 }]
            })
        );

        assert_eq!(
            Value::FloatV { f_v: 0.0 }, 
            interp_expr(&BuiltinFn { 
                name: Builtin::Modulus,
                args: vec![Float { f: 2.0 }, Float { f: 2.0 }]
            })
        );

        assert_eq!(
            Value::IntV { i_v: 0 }, 
            interp_expr(&BuiltinFn { 
                name: Builtin::Modulus,
                args: vec![Integer { i: 2 }, Integer { i: 2 }]
            })
        );
    }

    #[test]
    fn floating_point_builtins() {
        assert_eq!(
            Value::IntV { i_v: 2 }, 
            interp_expr(&BuiltinFn { 
                name: Builtin::Truncate,
                args: vec![Integer { i: 2 }]
            })
        );

        assert_eq!(
            Value::FloatV { f_v: 2.0 }, 
            interp_expr(&BuiltinFn { 
                name: Builtin::Truncate,
                args: vec![Float { f: 2.1938 }]
            }),
        );

        assert_eq!(
            Value::IntV { i_v: 2 }, 
            interp_expr(&BuiltinFn { 
                name: Builtin::Round,
                args: vec![Integer { i: 2 }]
            })
        );

        assert_eq!(
            Value::FloatV { f_v: 2.0 }, 
            interp_expr(&BuiltinFn { 
                name: Builtin::Round,
                args: vec![Float { f: 2.1938 }]
            })
        );

        assert_eq!(
            Value::FloatV { f_v: 0.0 }, 
            interp_expr(&BuiltinFn { 
                name: Builtin::Fraction,
                args: vec![Integer { i: 2 }]
            })
        );

        assert_eq!(
            Value::FloatV { f_v: 2.1938f64.fract() }, 
            interp_expr(&BuiltinFn { 
                name: Builtin::Fraction,
                args: vec![Float { f: 2.1938 }]
            })
        );        
    }

    #[test]
    fn trig_bulitins() {
        assert_eq!(
            Value::FloatV { f_v: f64::sin(0.33) as f64 }, 
            interp_expr(&BuiltinFn { 
                name: Builtin::Sin,
                args: vec![Float { f: 0.33 }]
            }));

        assert_eq!(
            Value::FloatV { f_v: f64::cos(0.33) as f64 }, 
            interp_expr(&BuiltinFn { 
                name: Builtin::Cos,
                args: vec![Float { f: 0.33 }]
            }));
    
        assert_eq!(
            Value::FloatV { f_v: f64::tan(0.33) as f64 }, 
            interp_expr(&BuiltinFn { 
                name: Builtin::Tan,
                args: vec![Float { f: 0.33 }]
            }));

        assert_eq!(
            Value::FloatV { f_v: f64::asin(0.33) as f64 }, 
            interp_expr(&BuiltinFn { 
                name: Builtin::ArcSin,
                args: vec![Float { f: 0.33 }]
            }));

        assert_eq!(
            Value::FloatV { f_v: f64::acos(0.33) as f64 }, 
            interp_expr(&BuiltinFn { 
                name: Builtin::ArcCos,
                args: vec![Float { f: 0.33 }]
            }));
    
        assert_eq!(
            Value::FloatV { f_v: f64::atan(0.33) as f64 }, 
            interp_expr(&BuiltinFn { 
                name: Builtin::ArcTan,
                args: vec![Float { f: 0.33 }]
            }));
    }

    #[test]
    fn hyperbolic_bulitins() {
        use std::f64::consts::E;
        assert_eq!(
            Value::FloatV { f_v: f64::sinh(1.0) as f64 }, 
            interp_expr(&BuiltinFn { 
                name: Builtin::Sinh,
                args: vec![Float { f: 1.0 }]
            }));

        assert_eq!(
            Value::FloatV { f_v: f64::cosh(1.0) as f64 }, 
            interp_expr(&BuiltinFn { 
                name: Builtin::Cosh,
                args: vec![Float { f: 1.0 }]
            }));
    
        assert_eq!(
            Value::FloatV { f_v: f64::tanh(1.0) as f64 }, 
            interp_expr(&BuiltinFn { 
                name: Builtin::Tanh,
                args: vec![Float { f: 1.0 }]
            }));

        assert_eq!(
            Value::FloatV { f_v: f64::asinh(E) as f64 }, 
            interp_expr(&BuiltinFn { 
                name: Builtin::ArcSinh,
                args: vec![Float { f: E }]
            }));

        assert_eq!(
            Value::FloatV { f_v: f64::acosh(E) as f64 }, 
            interp_expr(&BuiltinFn { 
                name: Builtin::ArcCosh,
                args: vec![Float { f: E }]
            }));
    
        assert_eq!(
            Value::FloatV { f_v: f64::atanh(1.0) as f64 }, 
            interp_expr(&BuiltinFn { 
                name: Builtin::ArcTanh,
                args: vec![Float { f: 1.0 }]
            }));
    }

    #[test]
    fn log_bulitins() {
        assert_eq!(
            Value::FloatV { f_v: f64::ln(0.33) as f64 }, 
            interp_expr(&BuiltinFn { 
                name: Builtin::Ln,
                args: vec![Float { f: 0.33 }]
            }));

        assert_eq!(
            Value::FloatV { f_v: f64::log2(0.33) as f64 }, 
            interp_expr(&BuiltinFn { 
                name: Builtin::Log2,
                args: vec![Float { f: 0.33 }]
            }));
    
        assert_eq!(
            Value::FloatV { f_v: f64::log10(0.33) as f64 }, 
            interp_expr(&BuiltinFn { 
                name: Builtin::Log10,
                args: vec![Float { f: 0.33 }]
            }));
    }

    #[test]
    fn pow_int_int() {
        assert_eq!(
            Value::IntV{i_v: 8}, 
            interp_expr(&BuiltinFn {
                name: Builtin::Pow, 
                args: vec![Integer { i: 2 }, Integer { i: 3 }]
            }));
    }

    #[test]
    fn pow_int_float() {
        assert_eq!(
            Value::FloatV { f_v: 8.0 }, 
            interp_expr(&BuiltinFn {
                name: Builtin::Pow, 
                args: vec![Integer { i: 2 }, Float { f: 3.0 }]
            }));
    }

    #[test]
    fn pow_float_int() {
        assert_eq!(
            Value::FloatV { f_v: 8.0 }, 
            interp_expr(&BuiltinFn {
                name: Builtin::Pow, 
                args: vec![Float { f: 2.0 }, Integer { i: 3 }]
            }));
    }

    #[test]
    fn pow_float_float() {
        assert_eq!(
            Value::FloatV{f_v: 8.0}, 
            interp_expr(&BuiltinFn {
                name: Builtin::Pow, 
                args: vec![Float { f: 2.0 }, Float { f: 3.0 }]
            }));
    }
}