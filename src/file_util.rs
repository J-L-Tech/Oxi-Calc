use std::{fs, path::PathBuf};

use rfd::FileDialog;


pub fn get_save_path() -> Option<PathBuf> {
    return FileDialog::new().set_title("Select Save Folder").pick_folder();
}

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

pub fn split_csv_by_column(column_count: usize, csv_str: &str) -> Vec<String> {
    let mut result_columns: Vec<String> = (0..column_count).map(|_| String::new()).collect();

    for line in csv_str.lines() {
        'item_loop: for (index, item) in line.split(',').enumerate() {
            if index > result_columns.len() - 1 {
                break 'item_loop;
            }
            result_columns[index].push_str(item);
            result_columns[index].push(',');
            result_columns[index].push('\n');
        }
    }

    return result_columns;
}