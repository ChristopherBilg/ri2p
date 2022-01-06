mod daemon;

use clap::Parser;
use log::{error, info};
use std::fs;

fn main() {
    // Initialize the logger utility
    match env_logger::try_init() {
        Ok(_) => info!("Successfully initialized logger."),
        Err(error) => {
            error!("Failed to initialize logger.");
            error!("{}", error.to_string());
            std::process::exit(1);
        }
    };

    // Parse the derived program arguments (CLI)
    let program_args = daemon::program_arguments::ProgramArguments::parse();
    info!("Successfully parsed program arguments.");

    // Load in configuration files
    let config_toml_filename = program_args.config_filename;

    let config_toml_data = match fs::read_to_string(config_toml_filename) {
        Ok(data) => {
            info!("Successfully read configuration file.");
            data
        }
        Err(error) => {
            error!("Failed to read configuration file.");
            error!("{}", error.to_string());
            std::process::exit(1);
        }
    };

    let config: daemon::config::Config = match toml::from_str(&config_toml_data) {
        Ok(data) => {
            info!("Successfully parsed configuration file.");
            data
        }
        Err(error) => {
            error!("Failed to parse configuration file.");
            error!("{}", error.to_string());
            std::process::exit(1);
        }
    };

    // Initialize and start daemon
    let daemon = daemon::daemon::Daemon { config };

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
