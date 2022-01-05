mod daemon;

use clap::Parser;
use log::{error, info};
use serde_derive::{Deserialize, Serialize};
use std::fs;
use toml;

#[derive(Parser)]
#[clap(about, version, author)]
struct ProgramArguments {
    /// The filename for ri2p configurations
    #[clap(short, long, default_value = ".ri2p.config.toml")]
    config_filename: String,
}

#[derive(Deserialize, Serialize)]
struct Config {
    hostname: String,
}

fn main() {
    // Initialize the logger utility
    match env_logger::try_init() {
        Ok(_) => info!("Successfully initialized logger."),
        Err(error) => {
            error!("Failed to initialize logger.");
            error!("{}", error.to_string());
            std::process::exit(1);
        },
    };

    // Parse the derived program arguments (CLI)
    let program_args = ProgramArguments::parse();
    info!("Successfully parsed program arguments.");

    // Load in configuration files
    let config_toml_filename = program_args.config_filename;

    let config_toml_data = match fs::read_to_string(config_toml_filename) {
        Ok(data) => {
            info!("Successfully read configuration Toml file.");
            data
        },
        Err(error) => {
            error!("Failed to read configuration Toml file.");
            error!("{}", error.to_string());
            std::process::exit(1);
        },
    };

    let _config: Config = match toml::from_str(&config_toml_data) {
        Ok(data) => {
            info!("Successfully parsed configuration Toml file.");
            data
        },
        Err(error) => {
            error!("Failed to parse configuration Toml file.");
            error!("{}", error.to_string());
            std::process::exit(1);
        },
    };

    // Initialize and start daemon
    let daemon = daemon::daemon::Daemon;

    if daemon.initialize() {
        info!("Successfully initialized daemon.");
    } else {
        error!("Failed to initialize daemon.");
        std::process::exit(1);
    }

    info!("Starting daemon.");
    daemon.start();

    info!("Stopping daemon.");
    daemon.stop();
}
