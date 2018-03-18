#[macro_use]
extern crate log;
extern crate env_logger;

use std::env;
use env_logger::{Builder, Target};

fn configure_logger() {
    let mut builder = Builder::new();

    builder.target(Target::Stdout);

    if !env::var("RUST_LOG").is_ok() {
        builder.parse("info");
    }

    builder.init();
}

fn main() {
    configure_logger();

    info!("Yalci is starting...");

    info!("Yalci has started");

    info!("Yalci is stopping...");

    info!("Yalci has stopped");
}
