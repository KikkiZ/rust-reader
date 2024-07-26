use log::error;

use crate::entity::bookinfo::BookInfo;

#[tauri::command]
pub fn book_list() -> String {
    let list = BookInfo::get_info_list();
    match list {
        Ok(list) => serde_json::to_string(&list).unwrap(),
        Err(err) => {
            error!("读取数据时出现错误: {}", err);
            panic!();
        }
    }
}
