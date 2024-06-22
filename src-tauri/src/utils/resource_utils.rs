use std::{
    env,
    fs::{create_dir_all, File},
    io::Write,
    path::PathBuf,
};

use crate::utils::config_utils::Config;

const DIR_LIST: [&str; 3] = ["book", "cover", "resources"];

/// 资源完整性检查
#[cfg(target_os = "windows")]
pub fn resource_integrity_check() {
    let mut dir = PathBuf::from(env::var("LOCALAPPDATA").unwrap());
    // let mut dir = PathBuf::from(env::var("APPDATA").unwrap());
    dir.push("Reader");

    // 检查目录完整性
    for name in DIR_LIST {
        dir_check(&mut dir, name);
    }

    // 检查相关文件完整性
    config_check(&mut dir);
    book_info_check(&mut dir);
}

fn dir_check(path: &mut PathBuf, name: &str) {
    path.push(name);

    if !path.exists() {
        println!("Create dir: {:?}", path);
        create_dir_all(&path).unwrap();
    }

    path.pop();
}

fn config_check(path: &mut PathBuf) {
    path.push("config.yml");

    if !path.exists() {
        println!("Create config.yml");
        let data_dir = env::var("LOCALAPPDATA").unwrap();
        let mut file = File::create(&path).expect("Failed to create config.yml");
        let mut config = Config::default();

        config.book.info = format!("{}\\Reader\\book_info.json", data_dir);
        config.book.dir = format!("{}\\Reader\\book", data_dir);
        config.book.cover = format!("{}\\Reader\\cover", data_dir);
        config.book.resources = format!("{}\\Reader\\resources", data_dir);

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
