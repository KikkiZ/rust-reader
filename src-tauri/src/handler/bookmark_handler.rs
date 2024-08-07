use log::warn;
use serde_json::json;

use crate::{
    entity::{
        bookmark::BookMark,
        notification::{Notification, NotificationType},
    },
    utils::common_utils::json_to_string,
};

#[tauri::command]
pub fn add_bookmark(data: &str) -> String {
    let mut success = false;

    match serde_json::from_str::<BookMark>(data) {
        Ok(mark) => {
            let mark = mark.update();
            if !BookMark::insert_mark(&mark) {
                warn!("添加书签失败");
            } else {
                success = true;
            }
        }
        Err(err) => {
            warn!("解析书签时发生错误: {}", err);
            warn!("书签数据: {}", data);
        }
    }

    let result;
    if success {
        result = json!({
            "success": true,
        });
    } else {
        let msg = Notification {
            r#type: NotificationType::Warn,
            title: "Warn".to_string(),
            msg: "添加书签失败, 详细信息请查看日志记录".to_string(),
        };

        result = json!({
            "success": false,
            "msg": msg,
        });
    }

    json_to_string(&result)
}

#[tauri::command]
pub fn get_chapter_mark_list(id: &str, chapter: usize) -> String {
    let result;

    match BookMark::get_mark_list_by_chapter(id, chapter) {
        Ok(list) => {
            result = json!({
                "success": true,
                "list": list,
            });
        }
        Err(err) => {
            let msg = Notification {
                r#type: NotificationType::Warn,
                title: "Warn".to_string(),
                msg: "获取书签信息时发生了错误".to_string(),
            };

            result = json!({
                "success": false,
                "msg":msg,
            });
            warn!("查询数据时发生了错误: {:?}", err);
        }
    }

    json_to_string(&result)
}

#[tauri::command]
pub fn delete_mark(id: usize) -> String {
    let result;

    match BookMark::remove_mark(id) {
        true => {
            result = json!({
                "success": true,
            });
        }
        false => {
            let msg = Notification {
                r#type: NotificationType::Warn,
                title: "Warn".to_string(),
                msg: "An error occurred while getting info".to_string(),
            };

            result = json!({
                "success": false,
                "msg":msg,
            });
            warn!("删除书签数据时发生了错误");
        }
    }

    json_to_string(&result)
}
