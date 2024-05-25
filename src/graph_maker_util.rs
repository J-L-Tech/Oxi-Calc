pub fn split_csv_by_column(column_count: usize, csv_str: &str) -> Vec<String> {
    let mut result_columns: Vec<String> = (0..column_count).map(|_| String::new()).collect();

    for line in csv_str.lines() {
        'item_loop: for (index, item) in line.split(',').enumerate() {
            if index > result_columns.len() - 1 {
                break 'item_loop;
            }
            result_columns[index].push_str(item);
            result_columns[index].push('\n');
        }
    }

    return result_columns;
}