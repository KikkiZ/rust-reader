use crate::utils::config_utils::GLOBAL_CONFIG;

#[tauri::command]
pub fn get_config() -> String {
    let config = GLOBAL_CONFIG.clone();
    let json = serde_json::to_string(&config).unwrap();

    json
}

#[tauri::command]
pub fn get_resource_path() -> String {
    GLOBAL_CONFIG.book.resources.clone()
}
