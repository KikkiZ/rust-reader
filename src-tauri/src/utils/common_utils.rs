use std::{
    fs::File,
    io::{Read, Write},
    path::PathBuf,
    time::SystemTime,
};

use crypto_hash::{Algorithm, Hasher};
use serde::Serialize;
use tauri::{Manager, Runtime};
use window_shadows::set_shadow;

pub fn set_window_shadow<R: Runtime>(app: &tauri::App<R>) {
    let window = app.get_window("main_window").unwrap();
    set_shadow(&window, true).expect("error while set window shadow");
}

pub fn hash(path: &PathBuf) -> String {
    let mut file = File::open(path).unwrap();

    let mut hasher = Hasher::new(Algorithm::SHA256);
    let mut buffer = [0; 2048];
    loop {
        let bytes = file.read(&mut buffer).unwrap();
        if bytes == 0 {
            break;
        }

        let _ = hasher.write(&mut buffer);
    }

    hasher
        .finish()
        .iter()
        .map(|byte| format!("{:02x}", byte))
        .collect::<Vec<String>>()
        .join("")
}

pub fn json_to_string<T>(data: &T) -> String
where
    T: Serialize,
{
    match serde_json::to_string(&data) {
        Ok(result) => result,
        Err(error) => {
            // TODO: Log error
            println!("{}", error);
            "".to_string()
        }
    }
}

/// Returns the current time in milliseconds
pub fn time_stamp() -> u64 {
    match SystemTime::now().duration_since(SystemTime::UNIX_EPOCH) {
        Ok(elapsed) => elapsed.as_millis() as u64,
        Err(_) => 0,
    }
}
