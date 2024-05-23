use std::{
    collections::HashMap,
    fs::{create_dir_all, File},
    io::{BufReader, Read, Write},
    path::{Path, PathBuf},
};

use epub::doc::EpubDoc;

use crate::{
    entity::{
        bookinfo::BookInfo,
        epub::Epub,
        notification::{Notification, NotificationType},
    },
    utils::{
        common_utils::{json_to_string, time_stamp},
        config_utils::GLOBAL_CONFIG,
    },
};

use super::CURRENT_BOOK;

// TODO: useless function
#[tauri::command]
pub fn click(id: &str) -> String {
    let books = BookInfo::get_info_list();
    let book = books.iter().find(|&book| book.id == id).unwrap();

    unsafe {
        CURRENT_BOOK = Some(Epub::new(&book.file_path));

        return CURRENT_BOOK.as_mut().unwrap().get_current_page();
    }
}

/// 打开书籍
///
/// 参数: id
///
/// 返回: 当前页面
///
/// TODO: 调整返回值
#[tauri::command]
pub fn open_book(id: &str) -> String {
    let mut books = BookInfo::get_info_list();

    match books.iter_mut().find(|book| book.id == id) {
        Some(info) => unsafe {
            let mut book = Epub::new(&info.file_path);

            info.last_open = time_stamp(); // 更新最后一次打开时间(time_stamp)
            book.info = info.clone(); // 同步bookinfo
            CURRENT_BOOK = Some(book);

            BookInfo::save_info_list(&books); // 保存更新后的信息
            return CURRENT_BOOK.as_mut().unwrap().get_current_page();
        },
        None => panic!("No such book"),
    }
}

/// 添加新书
///
/// 参数: Vec<&str> 新书路径
///
/// 返回一个Json Array, 内容为 Notification
#[tauri::command]
pub fn update_new_book(paths: Vec<&str>) -> String {
    let mut messages = Vec::new();
    let mut list = BookInfo::get_info_list();
    let mut title_list = Vec::new();

    for path in paths {
        let path = Path::new(path);

        if !path.is_file() {
            continue;
        }

        let mut info = BookInfo::new(path.to_path_buf());
        let mut book = EpubDoc::new(path).unwrap();

        info.last_open = time_stamp();

        if list.contains(&info) {
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

        list.push(info);
    }

    BookInfo::save_info_list(&list);

    if title_list.len() != 0 {
        messages.push(Notification {
            r#type: NotificationType::Info,
            title: "Info".to_string(),
            msg: format!("Update {} success.", title_list.join(", ")),
        })
    }

    json_to_string(&messages)
}

fn save_cover(info: &BookInfo, book: &mut EpubDoc<BufReader<File>>) {
    let (cover, mime) = book.get_cover().unwrap();

    let mime: mime::Mime = mime.parse().unwrap();
    let mut path = PathBuf::from(GLOBAL_CONFIG.book.cover.clone());
    let cover_name = info.id.clone() + "." + mime.subtype().as_str();
    path.push(cover_name);

    println!("{:?}", path);
    // TODO: 新建文件错误, 需要手动处理错误, 否则会panic
    let mut file = File::create(path).unwrap();
    let _ = file.write_all(&cover);
}

fn save_book(info: &BookInfo, path: &str) {
    let book_name = info.id.clone() + ".epub";
    let mut book_path = PathBuf::from(GLOBAL_CONFIG.book.dir.clone());
    book_path.push(book_name);

    let mut src_file = File::open(path).unwrap();
    let mut dest_file = File::create(book_path).unwrap();
    let mut buffer = Vec::new();
    let _ = src_file.read_to_end(&mut buffer);
    let _ = dest_file.write_all(&buffer);
}

fn save_resources(info: &BookInfo, book: &mut EpubDoc<BufReader<File>>) {
    // let mut image_list = Vec::new();
    let resources = book.resources.clone();
    let mut resources_path = PathBuf::from(&GLOBAL_CONFIG.book.resources.clone());
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

#[tauri::command]
pub fn get_css() -> String {
    let mut result = HashMap::new();

    unsafe {
        match CURRENT_BOOK.as_mut() {
            Some(book) => {
                let css_list = book.get_css();
                result.insert("css", json_to_string(&css_list));
                result.insert("success", String::from("true"));
            }
            None => {
                result.insert("success", String::from("false"));
            }
        }
    }

    json_to_string(&result)
}
