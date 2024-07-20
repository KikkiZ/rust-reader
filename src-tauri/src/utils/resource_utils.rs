use std::{
    collections::HashMap,
    env,
    fs::{create_dir_all, File},
    io::Write,
    path::PathBuf,
};

use rusqlite::Connection;

use crate::utils::config_utils::Config;

const DIR_LIST: [&str; 3] = ["book", "cover", "resources"];

#[cfg(target_os = "windows")]
fn data_dir() -> PathBuf {
    // PathBuf::from(env::var("APPDATA").unwrap())
    let mut path = PathBuf::from(env::var("LOCALAPPDATA").unwrap());
    path.push("Reader");

    path
}

#[cfg(target_os = "linux")]
fn data_dir() -> PathBuf {
    let mut path = PathBuf::from(env::var("HOME").unwrap());
    path.push(".Reader");

    path
}

/// 资源完整性检查
pub fn resource_integrity_check() {
    let mut dir = data_dir();

    // 检查目录完整性
    dir_check(&mut dir);

    // 检查相关文件完整性
    config_check(&mut dir);
    book_info_check(&mut dir);
    database_check(&mut dir);
}

fn dir_check(path: &mut PathBuf) {
    for name in DIR_LIST {
        path.push(name);

        if !path.exists() {
            println!("Create dir: {:?}", path);
            create_dir_all(&path).unwrap();
        }

        path.pop();
    }
}

fn config_check(path: &mut PathBuf) {
    path.push("config.yml");

    if !path.exists() {
        println!("Create config.yml");
        let mut file = File::create(&path).expect("Failed to create config.yml");
        let mut config = Config::default();
        let dir = {
            let mut temp = path.clone();
            temp.pop();

            temp.to_str().unwrap().to_owned()
        };

        config.database = format!("{}\\data.sqlite", dir);
        config.book.info = format!("{}\\book_info.json", dir);
        config.book.dir = format!("{}\\book", dir);
        config.book.cover = format!("{}\\cover", dir);
        config.book.resources = format!("{}\\resources", dir);

        file.write_all(serde_yml::to_string(&config).unwrap().as_bytes())
            .expect("Something wrong with config.yml");
    }

    path.pop();
}

fn book_info_check(path: &mut PathBuf) {
    path.push("book_info.json");

    if !path.exists() {
        println!("Create book_info.json");
        let mut file = File::create(&path).expect("Failed to create book_info.json");
        let _ = file.write_all(r#"[]"#.as_bytes());
    }

    path.pop();
}

fn database_check(path: &mut PathBuf) {
    path.push("data.sqlite");

    let conn = match Connection::open(&path) {
        Ok(conn) => conn,
        Err(_) => panic!("Failed to create db.sqlite"),
    };

    let mut tables: HashMap<&str, &str> = HashMap::new();
    tables.insert(
        "book_info",
        "CREATE TABLE book_info (
                id          TEXT    PRIMARY KEY,
                file_path   TEXT    NOT NULL,
                cover_path  TEXT    NOT NULL,
                title       TEXT    NOT NULL,
                creator     TEXT    NOT NULL,
                date        TEXT    NOT NULL,
                publisher   TEXT    NOT NULL,
                language    TEXT    NOT NULL,
                subject     TEXT    NOT NULL,
                description TEXT    NOT NULL,
                last_open   INTEGER NOT NULL
            );",
    );

    for (name, sql) in tables {
        if table_check(&conn, name) {
            println!("Table {} exist", name);
            continue;
        } 
        
        // TODO: 优化出现错误时处理方法
        conn.execute(sql, []).expect("Failed to create table");
    }                       

    let _ = conn.close();
    path.pop();
}

fn table_check(conn: &Connection, table_name: &str) -> bool {
    let sql = "SELECT name FROM sqlite_master WHERE type='table' AND name= ?1";

    let mut stmt = conn.prepare(sql).unwrap();
    let mut rows = stmt.query([table_name]).unwrap();

    if let Some(_) = rows.next().unwrap() {
        return true;
    }

    false
}
