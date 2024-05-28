#[derive(Clone, Debug, PartialEq)]
pub enum Expr {
    Integer {i: i64},
    Float {f: f64},

    BuiltinFn {name: Builtin, args: Vec<Expr>},
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Builtin {
    // Basic Operations
    Add,
    Subtract,
    Multiply,
    Divide,
    PercentOf,
    Modulus,

    // General
    Abs,

    // Integer
    GCD,
    Choose,
    Permutation,

    // Decimal Point Manipulatoin
    Round,
    Truncate,
    Fraction,

    // Trig
    Sin,
    Cos,
    Tan,
    ArcSin,
    ArcCos,
    ArcTan,

    // Hyperbolic
    Sinh,
    Cosh,
    Tanh,
    ArcSinh,
    ArcCosh,
    ArcTanh,

    // Logarithms
    Ln,
    Log10,
    Log2,

    // Exponentiation
    Pow,

    // Selection
    Max,
    Min,

    // RNG
    Rand,
    RandInt,

}

#[derive(Clone, Debug, PartialEq)]
pub enum  Value {
    IntV {i_v: i64},
    FloatV {f_v: f64},
    Error {msg: String}
}

impl From<Value> for SharedString {
    fn from(value: Value) -> Self {
        match value {
            Value::IntV { i_v } => i_v.to_string().into(),
            Value::FloatV { f_v } => f_v.to_string().into(),
            Value::Error { msg } => msg.into(),
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ValuePair {
    IntInt {a: i64, b: i64},
    FloatInt {a: f64, b: i64},
    IntFloat {a: i64, b: f64},
    FloatFloat {a: f64, b: f64},
}

pub fn pair_up(a_v: Value, b_v: Value) -> ValuePair {
    match a_v {
        Value::IntV { i_v } => {
            let a_iv: i64 = i_v;
            match b_v {
                Value::IntV { i_v } => ValuePair::IntInt { a: a_iv, b: i_v },
                Value::FloatV { f_v } => ValuePair::IntFloat { a: a_iv, b: f_v },
                _ => panic!()
            }
        },
        Value::FloatV { f_v } => {
            let a_fv: f64 = f_v;
            match b_v {
                Value::IntV { i_v } => ValuePair::FloatInt { a: a_fv, b: i_v },
                Value::FloatV { f_v } => ValuePair::FloatFloat { a: a_fv, b: f_v },
                _ => panic!()
            }
        },
        _ => panic!()
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct ParseError {
    pub message: String,
}

impl ParseError {
    fn new(message: &str) -> ParseError {
        return ParseError {message: message.to_string()};
    }
}

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.message)
    }
}
impl Error for ParseError {}

impl From<String> for ParseError {
    fn from(value: String) -> Self {
        return ParseError::new(&value);
    }
}

impl From<&str> for ParseError {
    fn from(value: &str) -> Self {
        return ParseError::new(value);
    }
}

use std::{error::Error, fmt};

use slint::SharedString;