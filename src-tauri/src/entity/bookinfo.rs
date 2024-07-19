use std::{
    collections::HashMap,
    fmt::Display,
    fs::{self},
    path::PathBuf,
};

use epub::doc::EpubDoc;
use serde::{Deserialize, Serialize};

use crate::utils::{common_utils::hash, config_utils::read_config};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BookInfo {
    /// Epub的唯一标识符
    pub id: String,

    /// Epub文件存储路径
    pub file_path: PathBuf,

    /// 封面存储路径
    pub cover_path: PathBuf,

    /// 书本标题
    pub title: String,

    /// 书本作者
    pub creator: String,

    /// 书本出版日期
    pub date: String,

    /// 书本出版社
    pub publisher: String,

    /// 书本语言
    pub language: String,

    /// 书本分类
    pub subject: String,

    /// 书本简介
    pub description: String,

    /// 本书上次打开的时间
    pub last_open: u64,
}

impl BookInfo {
    pub fn new(path: PathBuf) -> Self {
        let hash_code = hash(&path);
        let mut book = EpubDoc::new(path).unwrap();

        let mut file_path = PathBuf::from(read_config().book.dir.clone());
        file_path.push(hash_code.clone() + ".epub");

        let cover_data = book.get_cover().unwrap();
        let mime: mime::Mime = cover_data.1.parse().unwrap();
        let subtype = mime.subtype().as_str();
        let mut cover_path = PathBuf::from(read_config().book.cover.clone());
        cover_path.push(hash_code.clone() + "." + subtype);

        let metadata = book.metadata.clone();

        BookInfo {
            id: hash_code,
            file_path,
            cover_path,
            title: get_value(&metadata, "title"),
            creator: get_value(&metadata, "creator"),
            date: get_value(&metadata, "date"),
            publisher: get_value(&metadata, "publisher"),
            language: get_value(&metadata, "language"),
            subject: get_value(&metadata, "subject"),
            description: get_value(&metadata, "description"),
            last_open: 0,
        }
    }

    pub fn get_info_list() -> Vec<BookInfo> {
        let book_info = read_config().book.info.clone();

        match fs::read_to_string(book_info) {
            Ok(content) => serde_json::from_str(&content).expect("Failed to parse JSON"),
            Err(_) => panic!(),
        }
    }

    pub fn save_info_list(list: &Vec<BookInfo>) {
        let json = serde_json::to_string(list).unwrap();
        let book_info = read_config().book.info.clone();
        std::fs::write(book_info, json).expect("Unable to write file");
    }
}

// 从metadata中获取值
fn get_value(data: &HashMap<String, Vec<String>>, key: &str) -> String {
    match data.get(key) {
        Some(value) => value.join(","),
        None => String::from(""),
    }
}

impl Display for BookInfo {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{{\n\tid: {},\n\ttitle: {},\n\tcreator: {},\n\tdate: {},\n\tlanguage: {},\n\tdescription: {}\n}}\n",
            self.id, self.title, self.creator, self.date, self.language, self.description
        )
    }
}

/// 手动实现 PartialEq trait 以实现BookInfo的比较
/// 比较 id 和 title, 如果相等则返回true
impl PartialEq for BookInfo {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id && self.title == other.title
    }
}
