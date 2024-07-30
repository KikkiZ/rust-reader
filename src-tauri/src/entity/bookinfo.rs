use std::{collections::HashMap, fmt::Display, path::PathBuf};

use epub::doc::EpubDoc;
use rusqlite::{params, Error};
use serde::{Deserialize, Serialize};

use crate::{
    utils::{common_utils::hash, config_utils::read_config},
    CONN,
};

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

    pub fn get_info_list() -> Result<Vec<BookInfo>, Error> {
        let conn = CONN.lock().unwrap();

        let sql = "SELECT * FROM book_info;";
        let mut stmt = conn.prepare(sql).unwrap();
        let rows = stmt
            .query_map([], |row| {
                Ok(BookInfo {
                    id: row.get(0).unwrap(),
                    file_path: PathBuf::from(row.get::<usize, String>(1).unwrap()),
                    cover_path: PathBuf::from(row.get::<usize, String>(2).unwrap()),
                    title: row.get(3).unwrap(),
                    creator: row.get(4).unwrap(),
                    date: row.get(5).unwrap(),
                    publisher: row.get(6).unwrap(),
                    language: row.get(7).unwrap(),
                    subject: row.get(8).unwrap(),
                    description: row.get(9).unwrap(),
                    last_open: row.get(10).unwrap(),
                })
            })
            .unwrap();

        let mut list = Vec::new();
        for row in rows {
            match row {
                Ok(info) => list.push(info),
                Err(err) => return Err(err),
            }
        }

        Ok(list)
    }

    pub fn get_specific_info(id: &str) -> Result<BookInfo, Error> {
        let conn = CONN.lock().unwrap();

        let sql = "SELECT * FROM book_info WHERE id = ?1;";
        conn.query_row(sql, [id], |row| {
            Ok(BookInfo {
                id: row.get(0).unwrap(),
                file_path: PathBuf::from(row.get::<usize, String>(1).unwrap()),
                cover_path: PathBuf::from(row.get::<usize, String>(2).unwrap()),
                title: row.get(3).unwrap(),
                creator: row.get(4).unwrap(),
                date: row.get(5).unwrap(),
                publisher: row.get(6).unwrap(),
                language: row.get(7).unwrap(),
                subject: row.get(8).unwrap(),
                description: row.get(9).unwrap(),
                last_open: row.get(10).unwrap(),
            })
        })
    }

    pub fn insert_info(info: &Self) -> bool {
        let conn = CONN.lock().unwrap();

        let sql = "INSERT INTO book_info ( 
                            id, 
                            file_path, 
                            cover_path, 
                            title, 
                            creator, 
                            date, 
                            publisher, 
                            language, 
                            subject, 
                            description, 
                            last_open
                        ) VALUES (
                            ?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11
                        );";
        let params = params![
            info.id,
            info.file_path.to_str().unwrap(),
            info.cover_path.to_str().unwrap(),
            info.title,
            info.creator,
            info.date,
            info.publisher,
            info.language,
            info.subject,
            info.description,
            info.last_open.to_string(),
        ];

        match conn.execute(sql, params) {
            Ok(_) => true,
            Err(_) => false,
        }
    }

    pub fn update_info(info: &Self) -> bool {
        let conn = CONN.lock().unwrap();

        let sql = "UPDATE book_info SET
                            file_path = ?1, 
                            cover_path = ?2, 
                            title = ?3, 
                            creator = ?4, 
                            date = ?5, 
                            publisher = ?6, 
                            language = ?7, 
                            subject = ?8, 
                            description = ?9, 
                            last_open = ?10
                        WHERE id = ?11;";
        let params = params![
            info.file_path.to_str().unwrap(),
            info.cover_path.to_str().unwrap(),
            info.title,
            info.creator,
            info.date,
            info.publisher,
            info.language,
            info.subject,
            info.description,
            info.last_open.to_string(),
            info.id,
        ];

        match conn.execute(sql, params) {
            Ok(1) => true,
            Ok(_) | Err(_) => false,
        }
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
