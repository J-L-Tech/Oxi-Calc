use slint::{SharedString, Weak};

slint::include_modules!();

#[derive(Copy, Clone, Debug)]
enum NumberFormat {
    Binary,
    Octal,
    Decimal,
    Hexadecimal,
}

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

fn display_as_all_formats(prefix: &str, raw_string: &str) -> () {
    let hex: String = number_as_format(parse_int::parse::<u64>(&format!("{}{}", prefix, raw_string)), NumberFormat::Hexadecimal);
    let dec: String = number_as_format(parse_int::parse::<u64>(&format!("{}{}", prefix, raw_string)), NumberFormat::Decimal);
    let oct: String = number_as_format(parse_int::parse::<u64>(&format!("{}{}", prefix, raw_string)), NumberFormat::Octal);
    let bin: String = number_as_format(parse_int::parse::<u64>(&format!("{}{}", prefix, raw_string)), NumberFormat::Binary);
    println!("{} {} {} {} {}", raw_string, hex, dec, oct, bin);
}

fn create_all_formats(prefix: &str, raw_string: &str) -> (String, String, String, String) {
    let hex: String = number_as_format(parse_int::parse::<u64>(&format!("{}{}", prefix, raw_string)), NumberFormat::Hexadecimal);
    let dec: String = number_as_format(parse_int::parse::<u64>(&format!("{}{}", prefix, raw_string)), NumberFormat::Decimal);
    let oct: String = number_as_format(parse_int::parse::<u64>(&format!("{}{}", prefix, raw_string)), NumberFormat::Octal);
    let bin: String = number_as_format(parse_int::parse::<u64>(&format!("{}{}", prefix, raw_string)), NumberFormat::Binary);
    return (bin, oct, dec, hex);
}

fn main() -> Result<(), slint::PlatformError> {
    let ui = AppWindow::new()?;
    //     w_ui.set_oct_str(new_vals.1.into());
    //     w_ui.set_dec_str(new_vals.2.into());
    //     w_ui.set_hex_str(new_vals.3.into());
    // });

    ui.global::<Conversion>().on_convert_to_bin(|prefix, raw_string| {
        println!("{}{}", prefix.to_string(), raw_string.to_string());
        return number_as_format(parse_int::parse::<u64>(&format!("{}{}", prefix, raw_string)), NumberFormat::Binary).into();
    });

    ui.global::<Conversion>().on_convert_to_oct(|prefix, raw_string| {
        return number_as_format(parse_int::parse::<u64>(&format!("{}{}", prefix, raw_string)), NumberFormat::Octal).into();
    });

    ui.global::<Conversion>().on_convert_to_dec(|prefix, raw_string| {
        return number_as_format(parse_int::parse::<u64>(&format!("{}{}", prefix, raw_string)), NumberFormat::Decimal).into();
    });

    ui.global::<Conversion>().on_convert_to_hex(|prefix, raw_string| {
        return number_as_format(parse_int::parse::<u64>(&format!("{}{}", prefix, raw_string)), NumberFormat::Hexadecimal).into();
    });

    ui.on_request_increase_value({
        let ui_handle = ui.as_weak();
        move || {
            let ui = ui_handle.unwrap();
            ui.set_counter(ui.get_counter() + 1);
        }
    });

    ui.run()
}
