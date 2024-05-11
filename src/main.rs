use std::fs;

slint::include_modules!();

#[derive(Copy, Clone, Debug)]
enum NumberFormat {
    Binary,
    Octal,
    Decimal,
    Hexadecimal,
}

// Number Conversion

fn number_as_format<T>(try_number: Result<u64, T>, format: NumberFormat) -> String {
    if let Ok(number) = try_number {
        match format {
            NumberFormat::Binary =>         format!("{:b}", number),
            NumberFormat::Octal =>          format!("{:o}", number),
            NumberFormat::Decimal =>        format!("{}", number),
            NumberFormat::Hexadecimal =>    format!("{:X}", number),
        }
    }
    else {
        return "".to_string();
    }
}

// Unit Conversion

// Time

fn to_seconds_conversion_factor(left_type: &str) -> f64 {
    match left_type {
        "Miliseconds" =>    1.0 / 1000.0,
        "Seconds" =>        1.0,
        "Minutes" =>        60.0 / 1.0,
        "Hours" =>          360.0 / 1.0 ,
        "Days" =>           86400.0 / 1.0 ,
        "Weeks" =>          604800.0 / 1.0 ,
        "Months" =>         7257600.0 / 1.0, 
        _ => f64::NAN
    }
}

fn from_seconds_conversion_factor(right_type: &str) -> f64 {
    match right_type {
        "Miliseconds" =>    1000.0 / 1.0 ,
        "Seconds" =>        1.0,
        "Minutes" =>        1.0 / 60.0,
        "Hours" =>          1.0 / 360.0,
        "Days" =>           1.0 / 86400.0,
        "Weeks" =>          1.0 / 604800.0,
        "Months" =>         1.0 / 7257600.0,
        _ => f64::NAN
    }
}

// Length

fn to_meters_conversion_factor(left_type: &str) -> f64 {
    match left_type {
        "Inches" => 0.0254,
        "Feet" => 0.3048,
        "Yards" => 0.9144,
        "Miles" => 1609.344,
        "mm" => 0.001,
        "cm" => 0.01,
        "dm" => 0.1,
        "m" => 1.0,
        "km" => 1000.0,
        _ => f64::NAN
    }
}

fn from_meters_conversion_factor(right_type: &str) -> f64 {
    match right_type {
        "Inches" =>     39.3700787,
        "Feet" =>       3.2808399,
        "Yards" =>      1.0936133,
        "Miles" =>      0.00062137,
        "mm" =>         1000.0,
        "cm" =>         100.0,
        "dm" =>         10.0,
        "m" =>          1.0,
        "km" =>         0.001,
        _ => f64::NAN
    }
}

// Weight

fn to_grams_conversion_factor(left_type: &str) -> f64 {
    match left_type {
        "Ounces" =>     28.3495231,
        "Pounds" =>     453.59237,
        "Tons" =>       1.0160E+6,
        "mg" =>         0.001,
        "g" =>          1.0,
        "kg" =>         1000.0,
        "Metric Tons" => 1000000.0,
        _ => f64::NAN
    }
}

fn from_grams_conversion_factor(right_type: &str) -> f64 {
    match right_type {
        "Ounces" =>     0.03527396,
        "Pounds" =>     0.00220462,
        "Tons" =>       9.8421E-7,
        "mg" =>         1000.0,
        "g" =>          1.0,
        "kg" =>         0.001,
        "Metric Tons" => 0.000001,
        _ => f64::NAN
    }
}

// All

fn conversion_factor(left_type: &str, right_type: &str) -> f64 {
    if left_type == right_type {
        1.0
    }
    else {
        match left_type {   
            "Miliseconds" | "Seconds" | "Minutes" | "Hours" | "Days" | "Weeks" | "Months" | "Years" =>
                to_seconds_conversion_factor(left_type) * from_seconds_conversion_factor(right_type),
            "Inches" | "Feet" | "Yards" | "Miles" | "mm" | "cm" | "dm" | "m" | "km" =>
                to_meters_conversion_factor(left_type) * from_meters_conversion_factor(right_type),
            "Ounces" | "Pounds" | "Tons" | "mg" | "g" | "kg" | "Metric Tons" =>
                to_grams_conversion_factor(left_type) * from_grams_conversion_factor(right_type),
            _ => f64::NAN
        }
    }
}

fn value_as_unit(left_type: &str, left_value: &str, right_type: &str) -> String {
    if let Ok(number) = parse_int::parse::<f64>(left_value) {
        return format!("{}", number * conversion_factor(left_type, right_type));
    }
    else {
        return "".to_string();
    }
}

// Statistics

fn data_from_csv() -> String {
    let mut result: String = "".to_string();
    use rfd::FileDialog;

    if let Some(file) = FileDialog::new().add_filter("Data File", &["csv"]).pick_file() {
        if let Ok(contents) = fs::read_to_string(file) {
            result = contents;
        }
    }
    return result
}

fn main() -> Result<(), slint::PlatformError> {
    let ui = AppWindow::new()?;

    // Number Conversion

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

    ui.on_convert_units(|left_unit, left_value, right_unit | {
        return value_as_unit(left_unit.as_str(), left_value.as_str(), right_unit.as_str()).into();
    });

    // Stats Calc

    ui.on_data_from_csv(|| {
        return data_from_csv().into();
    });

    ui.run()
}
