use crate::entity::epub::Epub;

mod book_handler;
mod book_list_handler;
mod bookmark_handler;
mod config_handler;
mod read_handler;

pub fn get_handlers() -> impl Fn(tauri::Invoke) {
    tauri::generate_handler![
        book_handler::book_detail,
        book_handler::open_book,
        book_handler::update_new_book,
        book_handler::search_book,
        book_handler::get_css,
        book_list_handler::book_list,
        bookmark_handler::add_bookmark,
        bookmark_handler::get_chapter_mark_list,
        bookmark_handler::delete_mark,
        config_handler::get_config,
        config_handler::update_config,
        config_handler::get_resource_path,
        read_handler::prev_page,
        read_handler::next_page,
        read_handler::jump_to_chapter,
        read_handler::get_book_catalog,
    ]
}

/// 当前打开的Epub
static mut CURRENT_BOOK: Option<Epub> = None;
