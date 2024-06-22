use std::{env, fs::File, io::Read};

use lazy_static::lazy_static;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, Clone, Default)]
pub struct Config {
    pub book: BookData,
    pub theme: Theme,
    pub setting: Setting,
}

#[derive(Debug, Deserialize, Serialize, Clone, Default)]
pub struct BookData {
    pub info: String,
    pub dir: String,
    pub cover: String,
    pub resources: String,
}

#[derive(Debug, Deserialize, Serialize, Clone, Default)]
pub struct Theme {
    pub appearance: Appearance,
}

#[derive(Debug, Deserialize, Serialize, Clone, Default)]
pub enum Appearance {
    #[default]
    Light,
    Dark,
    System,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Setting {
    pub sidebar: bool,
}

impl Default for Setting {
    fn default() -> Self {
        Self {
            sidebar: true,
        }
    }
}

lazy_static! {
    pub static ref GLOBAL_CONFIG: Config = {
        let config_path = format!("{}\\Reader\\config.yml", env::var("LOCALAPPDATA").unwrap());
        let mut file = File::open(config_path).expect("Failed to open config file.");
        let mut content = String::new();
        file.read_to_string(&mut content)
            .expect("Failed to read file");

        serde_yml::from_str(&content).expect("Failed to parse YAML")
    };
}
