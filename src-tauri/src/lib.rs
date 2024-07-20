use std::sync::Mutex;

use lazy_static::lazy_static;
use rusqlite::Connection;

use utils::config_utils::read_config;

pub mod entity;
pub mod handler;
pub mod utils;

lazy_static! {
    static ref CONN: Mutex<Connection> = {
        let path = read_config().database;
        println!("lazy init: {}", path);
        let conn = Connection::open(path).unwrap();
        Mutex::new(conn)
    };
}
