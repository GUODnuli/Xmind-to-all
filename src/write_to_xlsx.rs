extern crate umya_spreadsheet;
use crate::sheet_to_tree::TestcaseTree;
use std::collections::HashMap;
pub fn write_xlsx(testcase_tree_data: TestcaseTree, xlsx_path: &str) {
    let mut book = umya_spreadsheet::reader::xlsx::read(xlsx_path).unwrap();

    let result = testcase_tree_data.get();

    for (index, case) in result.iter().enumerate() {
        for data_type in [("Path", "A"), ("Title", "B"), ("Marker", "I"), ("Step", "E"), ("Result", "F")].iter() {
            let (data_type, cell) = data_type;
            insert_cell(&mut book, data_type, cell, index, case);
        }
    }

    umya_spreadsheet::writer::xlsx::write(&book, xlsx_path).unwrap();
}
fn insert_cell(book: &mut umya_spreadsheet::Spreadsheet, data_type: &str, cell: &str, index: usize, case: &HashMap<String, String>) {
    if let Some(sheet) = book.get_sheet_mut(&0) {
        if let Some(cell_value) = case.get(data_type) {
            sheet.get_cell_mut(format!("{}{}", cell, index + 2))
                .set_value(cell_value);
        } else {
            eprintln!("Data type {} not found in case", data_type);
        }
    } else {
        eprintln!("Sheet 0 not found");
    }
}