use std::fs;

use crate::utils::config_utils::GLOBAL_CONFIG;

#[tauri::command]
pub fn book_list() -> String {
    let book_info = GLOBAL_CONFIG.book.info.clone();

    match fs::read_to_string(book_info) {
        Ok(content) => content,
        Err(_) => panic!(),
    }
}
