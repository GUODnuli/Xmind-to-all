extern crate umya_spreadsheet;

use std::collections::HashMap;
use std::path::PathBuf;
use std::sync::{Arc, Mutex};
use crate::sheet_to_tree::TestcaseTree;
// use crate::user_config::get_user_config;

pub fn write_xlsx(testcase_tree_data: TestcaseTree, xlsx_path: &PathBuf, user_config_data: &Arc<Mutex<HashMap<String, String>>>) {
    let mut book = umya_spreadsheet::reader::xlsx::read(&xlsx_path).unwrap();
    
    let user_config_data = user_config_data.lock().unwrap().clone();

    let result = testcase_tree_data.get();
    let result_len = result.len();
    // 列索引，从0开始，3表示D列
    let config_column_vec = vec![(4, "condition"), (7, "case_type"), (8, "case_state"), (10, "user_name")];
    for i in config_column_vec {
        let (column_index, config_key) = i;
        if let Some(sheet) = book.get_sheet_mut(&0) {
            for j in 0..result_len {
                sheet.get_cell_mut((column_index as u32, j as u32 + 2))
                    .set_value(user_config_data.get(config_key).unwrap());
            }
        }
    }

    for (index, case) in result.iter().enumerate() {
        for data_type in [
            ("Path", "A"),
            ("Title", "B"), 
            ("Step", "E"), 
            ("Result", "F"), 
            ("Marker", "I")
        ].iter() {
            let (data_type, cell) = data_type;
            insert_cell(&mut book, data_type, cell, index, case);
        }
    }

    umya_spreadsheet::writer::xlsx::write(&book, &xlsx_path).unwrap();
}

fn insert_cell(
    book: &mut umya_spreadsheet::Spreadsheet,
    data_type: &str, 
    cell: &str, 
    index: usize, 
    case: &HashMap<String, String>
) {
    if let Some(sheet) = book.get_sheet_mut(&0) {
        if let Some(cell_value) = case.get(data_type) {
            sheet.get_cell_mut(format!("{}{}", cell, index + 2))
                .set_value(cell_value);
        } else {
            eprintln!("Data type {} not found in case", data_type);
        }
    } else {
        eprintln!("未找到Sheet 0");
    }
}