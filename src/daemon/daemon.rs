use crate::daemon::config::Config;

pub struct Daemon {
    pub config: Config,
}

impl Daemon {
    pub fn initialize(&self) -> bool {
        true
    }

    pub fn start(&self) {}

    pub fn stop(&self) {}
}
