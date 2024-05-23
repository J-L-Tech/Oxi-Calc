use math_utils_lib::{eval, export, parse, quick_eval, solve, ExportType, MathLibError, StepType, Value, Variable};

pub fn calc_expr(raw_expr: &str) -> String {
    match parse_and_evaluate(raw_expr.into()) {
        Ok(value)          => value.to_string(),
        Err(error)  => error.get_reason()
    }
}

fn parse_and_evaluate(mut expr: String) -> Result<Value, MathLibError> {
    expr = expr.trim().split(" ").filter(|s| !s.is_empty()).collect();
    let parsed = parse(expr)?;
    let res = eval(&parsed, &Vec::new())?;
    return Ok(res);
}