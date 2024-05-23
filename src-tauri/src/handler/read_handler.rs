//! 处理阅读内容请求的一些方法
use serde_json::json;

use crate::{
    entity::notification::{Notification, NotificationType},
    utils::common_utils::json,
};

use super::CURRENT_BOOK;

/// 获取书籍目录
///
/// 返回一个Json Object：
/// {
///     "catalog": string,
///     "success": boolean,
///     "error": string
/// }
#[tauri::command]
pub fn get_book_catalog() -> String {
    let result;

    unsafe {
        match CURRENT_BOOK.as_mut() {
            Some(book) => {
                result = json!({
                    "success": true,
                    "catalog": book.get_catalog(),
                });
            }
            None => {
                // TODO:
                // 1. Log error
                // 2. 完善错误信息
                let error = Notification {
                    r#type: NotificationType::Err,
                    title: "Error".to_string(),
                    msg: "Has not open any book".to_string(),
                };

                result = json!({
                    "success": false,
                    "error": error,
                });
            }
        }
    }

    json(&result)
}

/// 获取上一章
///
/// 返回一个Json Object:
/// {
///     "content": string,
///     "exist": boolean,
///     "msg": string
/// }
#[tauri::command]
pub fn prev_page() -> String {
    let result;

    unsafe {
        match CURRENT_BOOK.as_mut() {
            Some(book) => {
                if book.go_prev() {
                    result = json!({
                        "exist": true,
                        "content": book.get_current_page(),
                    });
                } else {
                    let msg = Notification {
                        r#type: NotificationType::Warn,
                        title: "WARN".to_string(),
                        msg: "No previous page".to_string(),
                    };

                    result = json!({
                        "exist": false,
                        "msg": msg,
                    });
                }
            }
            None => {
                // TODO:
                // 1. Log error
                // 2. 完善错误信息
                let msg = Notification {
                    r#type: NotificationType::Err,
                    title: "Error".to_string(),
                    msg: "Has not open any book".to_string(),
                };

                result = json!({
                    "exist": false,
                    "msg": msg,
                })
            }
        }
    }

    println!("{}", json(&result));
    json(&result)
}

/// 获取下一章
///
/// 返回一个Json Object:
/// {
///     "content": string,
///     "exist": boolean,
///     "msg": string
/// }
#[tauri::command]
pub fn next_page() -> String {
    let result;

    unsafe {
        match CURRENT_BOOK.as_mut() {
            Some(book) => {
                if book.go_next() {
                    result = json!({
                        "exist": true,
                        "content": book.get_current_page(),
                    });
                } else {
                    let msg = Notification {
                        r#type: NotificationType::Warn,
                        title: "WARN".to_string(),
                        msg: "No next page".to_string(),
                    };

                    result = json!({
                        "exist": false,
                        "msg": msg,
                    });
                }
            }
            None => {
                // TODO:
                // 1. Log error
                // 2. 完善错误信息
                let msg = Notification {
                    r#type: NotificationType::Err,
                    title: "Error".to_string(),
                    msg: "Has not open any book".to_string(),
                };

                result = json!({
                    "exist": false,
                    "msg": msg,
                });
            }
        }
    }

    json(&result)
}

/// 跳转到指定的章节
///
/// 参数: usize 需要跳转到的章节
///
/// 返回一个 Json Object：
/// {
///     "content": string,
///     "exist": boolean,
///     "msg": string
/// }
#[tauri::command]
pub fn jump_to_chapter(chapter: usize) -> String {
    let result;

    unsafe {
        match CURRENT_BOOK.as_mut() {
            Some(book) => {
                if book.set_current_page(chapter) {
                    result = json!({
                        "exist": true,
                        "content": book.get_current_page(),
                    });
                } else {
                    let msg = Notification {
                        r#type: NotificationType::Warn,
                        title: "WARN".to_string(),
                        msg: "Page not found".to_string(),
                    };

                    result = json!({
                        "exist": false,
                        "msg": msg,
                    });
                }
            }
            None => {
                // TODO:
                // 1. Log error
                // 2. 完善错误信息
                let msg = Notification {
                    r#type: NotificationType::Err,
                    title: "Error".to_string(),
                    msg: "Has not open any book".to_string(),
                };

                result = json!({
                    "exist": false,
                    "msg": msg,
                });
            }
        }
    }

    json(&result)
}
