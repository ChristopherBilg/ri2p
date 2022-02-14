use std::{thread, time::Duration};

use crate::daemon::config::Config;

pub struct NetDB {
    pub config: Config,
}

impl NetDB {
    pub fn start(&self) {
        // https://github.com/PurpleI2P/i2pd/blob/openssl/libi2pd/NetDb.cpp#L49
        thread::sleep(Duration::from_millis(1000));
    }
}
