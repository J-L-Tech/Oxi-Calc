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

// Temperature

fn to_celcius_conversion_factor(left_type: &str) -> f64 {
    match left_type {
        "Fahrenheit" =>  0.5555555555555555555555555,
        "Celcius" =>     1.0,
        "Kelvin" =>      1.0,
        _ => f64::NAN
    }
}

fn from_celcius_conversion_factor(right_type: &str) -> f64 {
    match right_type {
        "Fahrenheit" =>  1.8,
        "Celcius" =>     1.0,
        "Kelvin" =>      1.0,
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
            "Fahrenheit" | "Celcius" | "Kelvin" =>
                to_celcius_conversion_factor(left_type) * from_celcius_conversion_factor(right_type),
            _ => f64::NAN
        }
    }
}

fn pre_offset_by_types(left_type: &str, right_type: &str) -> f64 {
    match (left_type, right_type) {
        ("Celcius", "Fahrenheit") =>    0.0,
        ("Celcius", "Kelvin") =>        0.0,
        ("Fahrenheit", "Celcius") =>    -32.0,
        ("Fahrenheit", "Kelvin") =>     -32.0,
        ("Kelvin", "Celcius") =>        0.0,
        ("Kelvin", "Fahrenheit") =>     -273.15,
        _ =>                            0.0,
    }
}

fn post_offset_by_types(left_type: &str, right_type: &str) -> f64 {
    match (left_type, right_type) {
        ("Celcius", "Fahrenheit") =>    32.0,
        ("Celcius", "Kelvin") =>        273.15,
        ("Fahrenheit", "Celcius") =>    0.0,
        ("Fahrenheit", "Kelvin") =>     273.15,
        ("Kelvin", "Celcius") =>        -273.15,
        ("Kelvin", "Fahrenheit") =>     32.0,
        _ =>                            0.0,
    }
}

pub fn value_as_unit(left_type: &str, left_value: &str, right_type: &str) -> String {
    if let Ok(number) = parse_int::parse::<f64>(left_value) {
        return format!("{}", 
            (number + pre_offset_by_types(left_type, right_type)) * conversion_factor(left_type, right_type) 
                + post_offset_by_types(left_type, right_type));
    }
    else {
        return "".to_string();
    }
}