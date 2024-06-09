pub mod expr_types;
pub mod interp_expr;
pub mod parse_expr;

use interp_expr::interp_expr;
use parse_expr::parse_expr;
use slint::SharedString;

pub fn evaluate_prefix_expression(raw_string: &str) -> SharedString {
    match parse_expr(raw_string) {
        Ok(expr) => interp_expr(&expr).into(),
        Err(e) => e.message.into(),
    }
}

pub mod func_expr_util {}
