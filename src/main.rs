#[macro_use]
extern crate log;
extern crate env_logger;

fn main() {
    env_logger::init();
    info!("Yalci is starting...");

    info!("Yalci has started");

    info!("Yalci is stopping...");

    info!("Yalci has stopped");
}
