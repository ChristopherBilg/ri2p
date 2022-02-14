use log::info;
use std::thread;

use crate::daemon::config::Config;
use crate::netdb::netdb::NetDB;

pub struct Daemon {
    pub config: Config,
}

impl Daemon {
    pub fn start(&self) {
        // Start the Network Database (NetDB)
        info!("Starting NetDB on its own thread.");
        let net_db = NetDB {
            config: self.config.clone(),
        };
        let net_db_thread = thread::spawn(move || {
            net_db.start();
        });

        //  Wait for all threads to finish execution before returning
        net_db_thread
            .join()
            .expect("The NetDB thread has panicked.");
    }
}
