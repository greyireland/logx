use std::thread::sleep;
use std::time::Duration;
use log::{debug, error, info, trace, warn};
use logo::{init_debug, init_prod};

#[test]
fn test_fn() {
    init_debug();
    debug!("hello");
    info!("nihao");
    trace!("skip");
    warn!("warn");
    error!("err");
}

#[test]
fn test_prd() {
    let _guard = init_prod();
    debug!("hello");
    info!("nihao");
    trace!("skip");
    warn!("warn");
    error!("err");
}