use std::io::{Error, Write};
use std::sync::{LazyLock, Mutex};

use ctor::ctor;
use flexi_logger::{
    Age, Cleanup, Criterion, DeferredNow, FileSpec, LogSpecification, Logger, Naming, WriteMode,
    TS_DASHES_BLANK_COLONS_DOT_BLANK,
};
use log::{error, info, LevelFilter, Record};
use rusqlite::Connection;

use handler::{book_handler, book_list_handler, bookmark_handler, config_handler, read_handler};
use utils::config_utils::read_config;
use utils::resource_utils::resource_integrity_check;

pub mod entity;
pub mod handler;
pub mod utils;

const MAX_LOG_AGE: Age = Age::Day;
const MAX_LOG_SIZE: u64 = 10 * 1024 * 1024;

static CONN: LazyLock<Mutex<Connection>> = LazyLock::new(|| {
    let path = read_config().database;
    let conn = Connection::open(path);
    match conn {
        Ok(conn) => {
            info!("数据库连接成功");
            Mutex::new(conn)
        }
        Err(err) => {
            error!("数据库连接失败: {}", err);
            panic!();
        }
    }
});

// 初始化程序相关资源
// 1. 检查资源完整性
// 2. 初始化日志
#[ctor]
fn initialize() {
    resource_integrity_check();

    let logspec = LogSpecification::builder()
        .default(LevelFilter::Error)
        .module("rust_reader", LevelFilter::Info)
        .build();
    let filespec = FileSpec::default()
        .directory(read_config().log)
        .suffix("log");

    Logger::with(logspec)
        .log_to_file(filespec)
        .rotate(
            Criterion::AgeOrSize(MAX_LOG_AGE, MAX_LOG_SIZE),
            Naming::TimestampsCustomFormat {
                current_infix: Some("current"),
                format: "%Y-%m-%d",
            },
            Cleanup::KeepLogFiles(3),
        )
        .write_mode(WriteMode::BufferAndFlush)
        .format_for_files(formatter)
        .start()
        .unwrap();

    info!("资源初始化成功");
}

fn formatter(w: &mut dyn Write, now: &mut DeferredNow, record: &Record) -> Result<(), Error> {
    write!(
        w,
        "[{}][ {:5} ] {}:{}: ",
        now.format(TS_DASHES_BLANK_COLONS_DOT_BLANK),
        record.level(),
        record.file().unwrap_or("<unnamed>"),
        record.line().unwrap_or(0),
    )?;

    write!(w, "{}", &record.args())
}

pub fn run() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            book_handler::book_detail,
            book_handler::open_book,
            book_handler::update_new_book,
            book_handler::search_book,
            book_handler::get_css,
            book_list_handler::book_list,
            bookmark_handler::add_bookmark,
            bookmark_handler::get_chapter_mark_list,
            bookmark_handler::delete_mark,
            config_handler::get_config,
            config_handler::update_config,
            config_handler::get_resource_path,
            read_handler::prev_page,
            read_handler::next_page,
            read_handler::jump_to_chapter,
            read_handler::get_book_catalog,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
