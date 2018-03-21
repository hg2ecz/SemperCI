#[macro_use]
extern crate log;
extern crate env_logger;
extern crate rusqlite;

mod configuration;
mod settings;

use std::env;
use env_logger::{Builder, Target};
use configuration::Configuration;

fn configure_logger() {
    let mut builder = Builder::new();

    builder.target(Target::Stdout);

    if env::var("RUST_LOG").is_ok() {
        builder.parse(&env::var("RUST_LOG").unwrap());
    } else {
        builder.parse("info");
    }

    builder.init();
}

fn main() {
    configure_logger();

    info!("Yalci is starting...");

    info!("Reading configuration...");
    match Configuration::new(settings::CONFIGURATION_DB) {
        Ok (configuration) => {
            info!("Watching repository: {}", configuration.repo_path);

            for branch in configuration.branches {
                info!("  * branch: {}", branch.name);
            }

            info!("Yalci has started.");
        },
        Err (error) => {
            error!("Could not load configuration: {:?}", error);
        }
    }

    info!("Yalci is stopping...");
    info!("Yalci has stopped.");
}
