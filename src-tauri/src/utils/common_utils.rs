use std::{
    fs::File,
    io::{Read, Write},
    path::PathBuf,
    time::SystemTime,
};

use crypto_hash::{Algorithm, Hasher};
use log::warn;
use serde::Serialize;

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
            warn!("解析 JSON 数据异常: {}", error);
            "".to_string()
        }
    }
}

/// 返回当前时间戳(ms)
pub fn time_stamp() -> u64 {
    match SystemTime::now().duration_since(SystemTime::UNIX_EPOCH) {
        Ok(elapsed) => elapsed.as_millis() as u64,
        Err(_) => 0,
    }
}
