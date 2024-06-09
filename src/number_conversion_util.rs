// Number Conversion

#[derive(Copy, Clone, Debug)]
pub enum NumberFormat {
    Binary,
    Octal,
    Decimal,
    Hexadecimal,
}

// #[derive(Copy, Clone, Debug)]
// pub enum FloatFormat {
//     Single,
//     Double,
//     Decimal
// }

pub fn number_as_format<T>(try_number: Result<u64, T>, format: NumberFormat) -> String {
    if let Ok(number) = try_number {
        match format {
            NumberFormat::Binary => format!("{:b}", number),
            NumberFormat::Octal => format!("{:o}", number),
            NumberFormat::Decimal => format!("{}", number),
            NumberFormat::Hexadecimal => format!("{:X}", number),
        }
    } else {
        return "".to_string();
    }
}

// pub fn float_as_format(try_number: Result<f64, T>, format: FloatFormat) -> String {
//     if let Ok(number) = try_number {
//         match format {
//             FloatFormat::Single => format!("{:b}", number),
//             FloatFormat::Double => format!("{:b}", number as f32),
//             FloatFormat::Decimal => format!("{}", number),
//         }
//     } else {
//         return "".to_string();
//     }
// }
