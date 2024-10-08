use std::{
    fs::{create_dir_all, File},
    io::{BufReader, Read, Write},
    path::{Path, PathBuf},
};

use epub::doc::EpubDoc;
use log::{error, warn};
use rusqlite::Error as SqlError;
use serde_json::json;

use crate::{
    entity::{
        bookinfo::BookInfo,
        epub::Epub,
        notification::{Notification, NotificationType},
    },
    utils::{
        common_utils::{json_to_string, time_stamp},
        config_utils::read_config,
    },
};

use super::CURRENT_BOOK;

/// 获取书籍详情
///
/// 参数: id
///
/// 返回一个Json Object:
/// {
///     "exist": boolean,
///     "info": string,
///     "msg": string
/// }
#[tauri::command]
pub fn book_detail(id: &str) -> String {
    let result;

    match BookInfo::get_specific_info(id) {
        Ok(info) => {
            result = json!({
                "exist": true,
                "info": info,
            })
        }
        Err(SqlError::QueryReturnedNoRows) => {
            let msg = Notification {
                r#type: NotificationType::Warn,
                title: "Warn".to_string(),
                msg: "No such book".to_string(),
            };

            result = json!({
                "exist": false,
                "msg": msg,
            })
        }
        Err(err) => {
            let msg = Notification {
                r#type: NotificationType::Err,
                title: "Error".to_string(),
                msg: "An error occurred while getting info".to_string(),
            };

            result = json!({
                "exist": false,
                "msg": msg,
            });

            warn!(
                "查询数据时发生了错误: {:?}",
                err.sqlite_error_code().unwrap()
            );
        }
    }

    json_to_string(&result)
}

/// 打开书籍
///
/// 参数: id
///
/// 返回一个Json Object:
/// {
///     "content": string,
///     "success": boolean,
///     "msg": string
/// }
#[tauri::command]
pub fn open_book(id: &str) -> String {
    let result;

    match BookInfo::get_specific_info(id) {
        Ok(mut info) => {
            let mut book = Epub::new(&info.file_path);

            info.last_open = time_stamp(); // 更新最后一次打开时间(time_stamp)
            book.info = info.clone(); // 同步bookinfo
            
            {
                // 需要在代码块中修改资源, 并在块结束时释放资源, 防止后续的死锁问题
                let mut container = CURRENT_BOOK.lock().unwrap();
                *container = Some(book);
            }

            BookInfo::update_info(&info); // 保存更新后的信息

            result = json!({
                "success": true,
                "content": CURRENT_BOOK.lock().unwrap().as_mut().unwrap().get_current_page(),
            });
        }
        // 查询不到指定数据
        Err(SqlError::QueryReturnedNoRows) => {
            let msg = Notification {
                r#type: NotificationType::Err,
                title: "Error".to_string(),
                msg: "No such book".to_string(),
            };

            result = json!({
                "success": false,
                "msg": msg,
            });
        }
        Err(err) => {
            error!(
                "查询数据时发生了错误: {:?}",
                err.sqlite_error_code().unwrap()
            );
            panic!("{}", err);
        }
    }

    json_to_string(&result)
}

/// 添加新书
///
/// 参数: Vec<&str> 新书路径
///
/// 返回一个Json Array, 内容为 Notification
#[tauri::command]
pub fn update_new_book(paths: Vec<&str>) -> String {
    let mut messages = Vec::new();
    let mut title_list = Vec::new();

    for path in paths {
        let path = Path::new(path);

        if !path.is_file() {
            continue;
        }

        let mut info = BookInfo::new(path.to_path_buf());
        let mut book = EpubDoc::new(path).unwrap();

        info.last_open = time_stamp();

        if !BookInfo::insert_info(&info) {
            messages.push(Notification {
                r#type: NotificationType::Warn,
                title: "Warn".to_string(),
                msg: format!("{} has been added.", info.title),
            });

            continue;
        } else {
            title_list.push((&info.title).clone());
        }

        save_cover(&info, &mut book);
        save_book(&info, path.to_str().unwrap());
        save_resources(&info, &mut book);
    }

    if title_list.len() != 0 {
        messages.push(Notification {
            r#type: NotificationType::Info,
            title: "Info".to_string(),
            msg: format!("Update [{}] success.", title_list.join(", ")),
        })
    }

    json_to_string(&messages)
}

fn save_cover(info: &BookInfo, book: &mut EpubDoc<BufReader<File>>) {
    let (cover, mime) = book.get_cover().unwrap();

    let mime: mime::Mime = mime.parse().unwrap();
    let mut path = PathBuf::from(read_config().book.cover.clone());
    let cover_name = info.id.clone() + "." + mime.subtype().as_str();
    path.push(cover_name);

    println!("{:?}", path);
    let mut file = File::create(path).unwrap();
    let _ = file.write_all(&cover);
}

fn save_book(info: &BookInfo, path: &str) {
    let book_name = info.id.clone() + ".epub";
    let mut book_path = PathBuf::from(read_config().book.dir.clone());
    book_path.push(book_name);

    let mut src_file = File::open(path).unwrap();
    let mut dest_file = File::create(book_path).unwrap();
    let mut buffer = Vec::new();
    let _ = src_file.read_to_end(&mut buffer);
    let _ = dest_file.write_all(&buffer);
}

fn save_resources(info: &BookInfo, book: &mut EpubDoc<BufReader<File>>) {
    let resources = book.resources.clone();
    let mut resources_path = PathBuf::from(read_config().book.resources.clone());
    resources_path = resources_path.join(info.id.as_str());

    for (_, (path, mime)) in resources.iter() {
        let mime: mime::Mime = mime.parse().unwrap();

        if mime.type_().as_str() != "image" {
            continue;
        }

        let image = resources_path.join(path);
        let mut target_path = image.clone();
        target_path.pop();

        if !path.exists() {
            let _ = create_dir_all(target_path);
        }

        let mut file = File::create(image).unwrap();

        let buff = book.get_resource_by_path(path).unwrap();
        let _ = file.write_all(&buff);
    }
    // image_list
}

// TODO: 重写函数
#[tauri::command]
pub fn search_book(key: &str) {
    println!("{}", key);
}

/// 获取当前打开书本的 css 文件
///
/// 返回一个 Json Object：
/// {
///     "css": string,
///     "success": boolean,
/// }
#[tauri::command]
pub fn get_css() -> String {
    let result;

    match CURRENT_BOOK.lock().unwrap().as_mut() {
        Some(book) => {
            let css_list = book.get_css();

            result = json!({
                "success": true,
                "css": css_list,
            });
        }
        None => {
            // TODO: 增加返回的信息
            result = json!({
                "success": false,
            });
        }
    }

    json_to_string(&result)
}
