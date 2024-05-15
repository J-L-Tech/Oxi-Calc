// Number Conversion

#[derive(Copy, Clone, Debug)]
pub enum NumberFormat {
    Binary,
    Octal,
    Decimal,
    Hexadecimal,
}

pub fn number_as_format<T>(try_number: Result<u64, T>, format: NumberFormat) -> String {
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