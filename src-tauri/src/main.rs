// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use rust_reader::{
    handler,
    utils::{common_utils::set_window_shadow, resource_utils::resource_integrity_check},
};

fn main() {
    resource_integrity_check();
    // println!("{:?}", rust_reader::utils::config_utils::GLOBAL_CONFIG.book);
    // println!("{:?}", GLOBAL_CONFIG.theme);
    // println!("{:?}", GLOBAL_CONFIG.setting);
    
    // TODO: 重构 bookinfo 存取方式

    tauri::Builder::default()
        .setup(|app| {
            if cfg!(windows) {
                set_window_shadow(app);
            }
            Ok(())
        })
        .invoke_handler(handler::get_handlers())
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
