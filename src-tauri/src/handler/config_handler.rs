use crate::utils::config_utils::{Config, read_config, save_config};

// TODO: 调整接口的返回值

#[tauri::command]
pub fn get_config() -> String {
    let config = read_config();
    let json = serde_json::to_string(&config).unwrap();

    json
}

#[tauri::command]
pub fn update_config(config: String) {
    let origin_config = read_config();
    let config: Config = serde_json::from_str(&config).unwrap();

    if origin_config != config {
        save_config(config);
    }
}

#[tauri::command]
pub fn get_resource_path() -> String {
    read_config().book.resources.clone()
}
