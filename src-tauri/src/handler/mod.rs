use crate::entity::epub::Epub;

pub mod book_handler;
pub mod book_list_handler;
pub mod config_handler;

pub fn get_handlers() -> impl Fn(tauri::Invoke) {
    tauri::generate_handler![
        book_handler::click,
        book_handler::open_book,
        book_handler::prev_page,
        book_handler::next_page,
        book_handler::update_new_book,
        book_handler::search_book,
        book_handler::get_book_catalog,
        book_handler::get_css,
        book_handler::jump_to_chapter,
        book_list_handler::book_list,
        config_handler::get_config,
        config_handler::get_resource_path,
    ]
}

/// 当前打开的Epub
static mut CURRENT_BOOK: Option<Epub> = None;
