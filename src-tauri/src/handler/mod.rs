use std::sync::{LazyLock, Mutex};

use crate::entity::epub::Epub;

pub mod book_handler;
pub mod book_list_handler;
pub mod bookmark_handler;
pub mod config_handler;
pub mod read_handler;

/// 当前打开的Epub
// static mut CURRENT_BOOK: Option<Epub> = None;
static CURRENT_BOOK: LazyLock<Mutex<Option<Epub>>> = LazyLock::new(|| Mutex::new(None));
