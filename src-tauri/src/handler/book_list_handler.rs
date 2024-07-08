use std::fs;

use crate::utils::config_utils::read_config;

#[tauri::command]
pub fn book_list() -> String {
    let book_info = read_config().book.info;

    match fs::read_to_string(book_info) {
        Ok(content) => content,
        Err(_) => panic!(),
    }
}
