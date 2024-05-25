use std::f64::consts::{E, PI, TAU};

use math_utils_lib::{eval, parse, /*export, quick_eval, solve, ExportType, StepType,*/ MathLibError, Value, Variable};

pub fn calc_expr(raw_expr: &str) -> String {
    match parse_and_evaluate(raw_expr.into()) {
        Ok(value)          => value.to_string(),
        Err(error)  => error.get_reason()
    }
}

fn parse_and_evaluate(mut expr: String) -> Result<Value, MathLibError> {
    expr = expr.trim().split(" ").filter(|s| !s.is_empty()).collect();
    let parsed = parse(expr)?;
    let res = eval(&parsed, &vec![ 
        Variable::new("pi".to_string(), Value::Scalar(PI)), 
        Variable::new("tau".to_string(), Value::Scalar(TAU)),
        Variable::new("e".to_string(), Value::Scalar(E)),   
    ])?;
    return Ok(res);
}