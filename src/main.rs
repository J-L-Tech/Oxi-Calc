slint::include_modules!();

mod statistics_util;
mod unit_conversion_util;
mod number_conversion_util;
mod expression_util;

fn main() -> Result<(), slint::PlatformError> {
    let ui = AppWindow::new()?;

    // Expression Calculator

    use expression_util::calc_expr;

    ui.on_evaluate_expression(|raw_string| {
        return calc_expr(raw_string.as_str()).into();
    }); 

    ui.on_append_history(|previous_history, raw_expr, raw_ans| { 
        let mut new_history = previous_history.to_string();
        new_history.push_str(&format!("{} = {}\n", raw_expr, raw_ans));       
        return new_history.into();
    }); 

    // Number Conversion
    use number_conversion_util::{number_as_format, NumberFormat};

    ui.on_convert_to_bin(|prefix, raw_string| {
        return number_as_format(parse_int::parse::<u64>(&format!("{}{}", prefix, raw_string)), NumberFormat::Binary).into();
    });
    
    ui.on_convert_to_oct(|prefix, raw_string| {
        return number_as_format(parse_int::parse::<u64>(&format!("{}{}", prefix, raw_string)), NumberFormat::Octal).into();
    });

    ui.on_convert_to_dec(|prefix, raw_string| {
        return number_as_format(parse_int::parse::<u64>(&format!("{}{}", prefix, raw_string)), NumberFormat::Decimal).into();
    });

    ui.on_convert_to_hex(|prefix, raw_string| {
        return number_as_format(parse_int::parse::<u64>(&format!("{}{}", prefix, raw_string)), NumberFormat::Hexadecimal).into();
    });

    // Unit Conversion
    use unit_conversion_util::value_as_unit;

    ui.on_convert_units(|left_unit, left_value, right_unit | {
        return value_as_unit(left_unit.as_str(), left_value.as_str(), right_unit.as_str()).into();
    });

    // Stats Calc
    use statistics_util::{data_from_csv, one_dimensional_statistics, data_to_vector};

    ui.on_data_from_csv(|| {
        return data_from_csv().into();
    });

    ui.on_one_dimensional_statistics(|raw_input | {
        let raw_numbers: String = raw_input.to_string();
        return match one_dimensional_statistics(&mut data_to_vector(raw_numbers.as_str())) {
            Ok(output) => output,
            Err(err_msg) => err_msg.message
        }.into()
    });

    ui.run()
}
