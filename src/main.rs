#![windows_subsystem = "windows"]

use slint::{SharedString, VecModel};

slint::include_modules!();

mod expression_util;
mod file_util;
mod func_expr_util;
mod graph_maker_util;
mod number_conversion_util;
mod statistics_util;
mod unit_conversion_util;

fn main() -> Result<(), slint::PlatformError> {
    let ui = AppWindow::new()?;
    use file_util::*;

    // Expression Calculator

    use expression_util::calc_expr;
    use func_expr_util::evaluate_prefix_expression;

    ui.on_evaluate_infix_expression(|raw_string| {
        return calc_expr(raw_string.as_str()).into();
    });

    ui.on_evaluate_prefix_expression(|raw_string| {
        return evaluate_prefix_expression(raw_string.as_str());
    });

    ui.on_append_history(|previous_history, raw_expr, raw_ans| {
        let mut new_history = previous_history.to_string();
        new_history.push_str(&format!("{} = {}\n", raw_expr, raw_ans));
        return new_history.into();
    });

    // Number Conversion
    use number_conversion_util::{number_as_format, NumberFormat};

    ui.on_convert_to_bin(|prefix, raw_string| {
        return number_as_format(
            parse_int::parse::<u64>(&format!("{}{}", prefix, raw_string)),
            NumberFormat::Binary,
        )
        .into();
    });

    ui.on_convert_to_oct(|prefix, raw_string| {
        return number_as_format(
            parse_int::parse::<u64>(&format!("{}{}", prefix, raw_string)),
            NumberFormat::Octal,
        )
        .into();
    });

    ui.on_convert_to_dec(|prefix, raw_string| {
        return number_as_format(
            parse_int::parse::<u64>(&format!("{}{}", prefix, raw_string)),
            NumberFormat::Decimal,
        )
        .into();
    });

    ui.on_convert_to_hex(|prefix, raw_string| {
        return number_as_format(
            parse_int::parse::<u64>(&format!("{}{}", prefix, raw_string)),
            NumberFormat::Hexadecimal,
        )
        .into();
    });

    // Unit Conversion
    use unit_conversion_util::value_as_unit;

    ui.on_convert_units(|left_unit, left_value, right_unit| {
        return value_as_unit(left_unit.as_str(), left_value.as_str(), right_unit.as_str()).into();
    });

    // Stats Calc
    use statistics_util::{data_to_vector, one_dimensional_statistics};

    ui.on_data_from_csv(|| {
        return data_from_csv().into();
    });

    ui.on_one_dimensional_statistics(|raw_input| {
        let raw_numbers: String = raw_input.to_string();
        return match one_dimensional_statistics(&mut data_to_vector(raw_numbers.as_str())) {
            Ok(output) => output,
            Err(err_msg) => err_msg.message,
        }
        .into();
    });

    // Graph Maker
    use graph_maker_util::make_graph;

    ui.on_csv_with_columns(|column_count| {
        if let Ok(count) = column_count.try_into() {
            return slint::ModelRc::new(VecModel::from(
                split_csv_by_column(count, &data_from_csv())
                    .into_iter()
                    .map(Into::into)
                    .collect::<Vec<SharedString>>(),
            ));
        } else {
            return slint::ModelRc::new(slint::VecModel::default());
        }
    });

    ui.on_make_graph(|graph_info| match make_graph(graph_info) {
        Ok(_) => {}
        Err(e) => {
            println!("Error Occured {}", e)
        }
    });

    ui.run()
}
