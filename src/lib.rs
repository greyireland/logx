use std::{env, io};

use tracing::Level;
use tracing_appender::non_blocking::WorkerGuard;

pub fn init_prod() -> WorkerGuard {
    let args: Vec<String> = env::args().collect();
    let p = std::path::Path::new(&args[0]);
    let file_name = format!(
        "{}.log",
        p.file_name().unwrap_or_default().to_str().unwrap()
    );
    let file_appender = tracing_appender::rolling::daily("./logs/", file_name);
    let (non_blocking, guard) = tracing_appender::non_blocking(file_appender);
    tracing_subscriber::fmt()
        .with_writer(non_blocking)
        .with_ansi(false)
        .with_target(false)
        .with_level(true)
        .with_line_number(false)
        .with_thread_names(false)
        .with_max_level(Level::INFO)
        .init();
    guard
}

pub fn init_debug() {
    tracing_subscriber::fmt()
        .with_writer(io::stderr)
        .with_ansi(true)
        .with_target(false)
        .with_level(true)
        .with_line_number(false)
        .with_thread_names(false)
        .with_max_level(Level::DEBUG)
        .init();
}

