use std::{
    env,
    fs::File,
    io::{Read, Write},
    sync::LazyLock,
};

use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, Clone, Default, PartialEq, Eq)]
pub struct Config {
    pub database: String,
    pub log: String,
    pub book: BookData,
    pub theme: Theme,
    pub setting: Setting,
}

#[derive(Debug, Deserialize, Serialize, Clone, Default, PartialEq, Eq)]
pub struct BookData {
    pub info: String,
    pub dir: String,
    pub cover: String,
    pub resources: String,
}

#[derive(Debug, Deserialize, Serialize, Clone, Default, PartialEq, Eq)]
pub struct Theme {
    pub appearance: Appearance,
}

#[derive(Debug, Deserialize, Serialize, Clone, Default, PartialEq, Eq)]
pub enum Appearance {
    #[default]
    Light,
    Dark,
    System,
}

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq, Eq)]
pub struct Setting {
    pub sidebar: bool,
}

impl Default for Setting {
    fn default() -> Self {
        Self { sidebar: true }
    }
}

static CONFIG_PATH: LazyLock<String> = LazyLock::new(|| {
    if cfg!(target_os = "windows") {
        format!("{}\\Reader\\config.yml", env::var("LOCALAPPDATA").unwrap())
    } else if cfg!(target_os = "linux") {
        format!("{}/.Reader/config.yml", env::var("HOME").unwrap())
    } else {
        panic!("unsupported platform")
    }
});

pub fn read_config() -> Config {
    let mut file = File::open(CONFIG_PATH.as_str()).expect("Failed to open config file.");
    let mut content = String::new();
    file.read_to_string(&mut content)
        .expect("Failed to read file");

    serde_yml::from_str(&content).expect("Failed to parse YAML")
}

pub fn save_config(config: Config) {
    let content = serde_yml::to_string(&config).unwrap();

    let mut file = File::create(CONFIG_PATH.as_str()).expect("Failed to create config file.");
    file.write_all(content.as_bytes())
        .expect("Failed to write to file");
}
