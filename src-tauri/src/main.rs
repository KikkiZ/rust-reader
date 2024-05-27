// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use rust_reader::{handler, utils::common_utils::set_window_shadow};

fn main() {
    // println!("{:?}", rust_reader::utils::config_utils::GLOBAL_CONFIG.book);
    // println!("{:?}", GLOBAL_CONFIG.theme);
    // println!("{:?}", GLOBAL_CONFIG.setting);

    tauri::Builder::default()
        .setup(|app| {
            set_window_shadow(app);
            Ok(())
        })
        .invoke_handler(handler::get_handlers())
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
