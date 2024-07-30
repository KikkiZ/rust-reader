use std::io::{Error, Write};
use std::sync::Mutex;

use ctor::ctor;
use flexi_logger::{
    Age, Cleanup, Criterion, DeferredNow, FileSpec, LogSpecification, Logger, Naming, WriteMode,
    TS_DASHES_BLANK_COLONS_DOT_BLANK,
};
use lazy_static::lazy_static;
use log::{error, info, LevelFilter, Record};
use rusqlite::Connection;

use utils::config_utils::read_config;
use utils::resource_utils::resource_integrity_check;

pub mod entity;
pub mod handler;
pub mod utils;

const MAX_LOG_AGE: Age = Age::Day;
const MAX_LOG_SIZE: u64 = 10 * 1024 * 1024;

lazy_static! {
    static ref CONN: Mutex<Connection> = {
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

    };
}

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
