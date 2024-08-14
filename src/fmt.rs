use ftlog::FtLogFormat;
use log::{Level, Record};
use std::fmt::Display;
pub struct SimpleFormatter;
impl FtLogFormat for SimpleFormatter {
    fn msg(&self, record: &Record) -> Box<dyn Send + Sync + std::fmt::Display> {
        Box::new(Msg {
            level: record.level(),
            args: format!("{}", record.args()),
        })
    }
}

struct Msg {
    level: Level,
    args: String,
}

impl Display for Msg {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&format!("{} {}", self.level, self.args))
    }
}
