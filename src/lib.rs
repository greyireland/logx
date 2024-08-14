use std::env;
use std::path::Path;

use ::log::LevelFilter;
use ftlog::appender::{FileAppender, Period};
use ftlog::LoggerGuard;
use crate::fmt::SimpleFormatter;

pub mod fmt;

pub fn init_prod() -> LoggerGuard {
    let args: Vec<String> = env::args().collect();
    let p = std::path::Path::new(&args[0]);
    let file_name = p.file_name();
    let dir = Path::new("logs");
    if !dir.exists() {
        std::fs::create_dir_all(&dir).unwrap();
    }
    let p = format!(
        "{}/{}.log",
        dir.to_str().unwrap(),
        file_name.unwrap_or_default().to_str().unwrap()
    );
    let _writer =
        FileAppender::rotate_with_expire(p, Period::Day, ftlog::appender::Duration::weeks(1));
    ftlog::Builder::new()
        .root(_writer)
        .format(SimpleFormatter)
        .max_log_level(LevelFilter::Info)
        .try_init()
        .unwrap()
}

pub fn init_debug() {
    ftlog::Builder::new()
        .format(SimpleFormatter)
        .max_log_level(LevelFilter::Debug)
        .try_init()
        .unwrap();
}
