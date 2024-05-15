// Statistics
use std::{cmp::Ordering, error::Error, fmt, fs};

#[derive(Debug, Clone, PartialEq)]
pub struct ErrorMessage {
    pub message: String,
}

impl ErrorMessage {
    fn new(message: &str) -> ErrorMessage {
        return ErrorMessage {message: message.to_string()};
    }
}

impl fmt::Display for ErrorMessage {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.message)
    }
}
impl Error for ErrorMessage {}

pub fn data_from_csv() -> String {
    let mut result: String = "".to_string();
    use rfd::FileDialog;

    if let Some(file) = FileDialog::new().add_filter("Data File", &["csv"]).pick_file() {
        if let Ok(contents) = fs::read_to_string(file) {
            result = contents;
        }
    }
    return result
}

pub fn data_to_vector(raw_data: &str) -> Vec<f64> {
    let mut result: Vec<f64> = Vec::new();
    for raw_number in raw_data.rsplit(',') {
        if let Ok(number) = parse_int::parse::<f64>(raw_number.trim()) {
            result.push(number);
        }
    }
    return result;
}

pub fn one_dimensional_statistics(data: &mut Vec<f64>) -> Result<String, ErrorMessage> {
    // Averages
    if data.contains(&f64::NAN) {
        return Err(ErrorMessage::new("Data Contains NaN"));
    }
    data.sort_unstable_by(|a, b| a.partial_cmp(b).unwrap_or(Ordering::Equal));
    let sum: f64 = data.into_iter().fold(0.0, |acc, x| acc + *x);
    let mean: f64 = sum / (data.len() as f64);
    let median: f64 = median(data)?;
    let mode: f64 = mode(data)?;
    let max: f64 = *data.last().unwrap_or(&f64::NAN);
    let min: f64 = *data.first().unwrap_or(&f64::NAN);

    return Ok(format!("Mean: {}\nMedian: {}\nMode: {}\nMax: {}\nMin: {}", mean, median, mode, max, min));
}

/// Assumes sorted data
fn median(data: &Vec<f64>) -> Result<f64, ErrorMessage> {
    // Even: Mean Average of 2 Middle Values
    // [0 1 2 3]
    if data.len() % 2 == 0 { 
        if data.len() == 0 {
            return Err(ErrorMessage::new("No Data"));
        }
        else {
            return Ok((data[data.len()/2 - 1] + data[data.len()/2]) / 2.0);
        }
    }
    // Odd: Middle Value
    // [1, 2, 3, 4, 5]
    else {
        return Ok(data[data.len() / 2]);
    }
}

///
/// Assumes sorted data
fn mode(data: &Vec<f64>) -> Result<f64, ErrorMessage> {
    if let Some(mut mode) = data.get(0) {
        let mut max_count: usize = 0;
        let mut current_value: f64 = *mode;
        let mut count: usize = 0;
        for number in data {
            // Current Mode Instance Encountered Increase it
            if *number == *mode {
                count += 1;
                if count > max_count {
                    max_count = count;
                }
            }
            // Non Mode Number Accumulating
            else if *number == current_value {
                count -= 1;
                if count == 0 {
                    mode = number;
                    max_count += 1;
                    count = max_count;
                }
            }
            // Previous Non Mode Number Counted, and Smaller than Mode
            else {
                    current_value = *number;
                    count = max_count;
            }

        }
        return Ok(*mode);
    }
    else {
        return Err(ErrorMessage::new("No Data"));
    }
}

#[cfg(test)]
mod statistics_tests {
    use super::mode;

    #[test]
    fn obvious_mode() {
        assert_eq!(Ok(0.0), mode(&vec![0.0, 0.0, 0.0, 0.0, 0.0]));
    }

    #[test]
    fn mode_switches() {
        assert_eq!(Ok(0.0), mode(&vec![1.1, 1.1, 1.1, 1.1, 0.0, 0.0, 0.0, 0.0, 0.0]));
    }

    #[test]
    fn mode_does_not_switch() {
        assert_eq!(Ok(0.0), mode(&vec![0.0, 0.0, 0.0, 0.0, 0.0, 1.1, 1.1, 1.1, 1.1,]));
    }

    #[test]
    fn mode_reset_count_retain() {
        assert_eq!(Ok(0.0), mode(&vec![0.0, 0.0, 0.0, 0.0, 0.0, 1.1, 1.1, 1.1, 1.1, 2.2, 2.2, 2.2, 2.2, 2.2]));
    }

    #[test]
    fn mode_reset_count_change() {
        assert_eq!(Ok(2.2), mode(&vec![0.0, 0.0, 0.0, 0.0, 0.0, 1.1, 1.1, 1.1, 1.1, 2.2, 2.2, 2.2, 2.2, 2.2, 2.2]));
    }

    #[test]
    fn mode_general_example() {
        assert_eq!(Ok(15.0), mode(&vec![3.0, 3.0, 6.0, 9.0, 15.0, 15.0, 15.0, 27.0, 27.0, 37.0, 48.0]));
    }
}