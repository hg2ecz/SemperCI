#[macro_use]
extern crate log;
extern crate env_logger;
extern crate rusqlite;

mod configuration;

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

    let configuration = Configuration::new("configuration.db").unwrap();

    info!("Yalci has started");

    info!("Yalci is stopping...");

    info!("Yalci has stopped");
}
