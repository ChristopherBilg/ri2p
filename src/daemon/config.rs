use serde_derive::{Deserialize, Serialize};

#[derive(Clone, Deserialize, Serialize)]
pub struct Config {
    pub reseed_hosts: Vec<String>,
}
