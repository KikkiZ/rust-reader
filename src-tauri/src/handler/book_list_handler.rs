use crate::entity::bookinfo::BookInfo;

#[tauri::command]
pub fn book_list() -> String {
    let list = BookInfo::get_info_list();
    match list {
        Ok(list) => serde_json::to_string(&list).unwrap(),
        // TODO: 这里需要处理错误
        Err(_) => panic!(),
    }
}
