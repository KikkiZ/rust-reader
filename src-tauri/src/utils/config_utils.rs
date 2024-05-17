use std::{fs::File, io::Read};

use lazy_static::lazy_static;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Config {
    pub book: BookData,
    pub theme: Theme,
    pub setting: Setting,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct BookData {
    pub info: String,
    pub dir: String,
    pub cover: String,
    pub resources: String,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Theme {
    pub appearance: Appearance,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub enum Appearance {
    Light,
    Dark,
    System,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Setting {
    pub sidebar: bool,
}

lazy_static! {
    pub static ref GLOBAL_CONFIG: Config = {
        let mut file = File::open("config.yml").expect("Failed to open config file.");
        let mut content = String::new();
        file.read_to_string(&mut content)
            .expect("Failed to read file");

        serde_yml::from_str(&content).expect("Failed to parse YAML")
    };
}
