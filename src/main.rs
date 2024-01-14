pub mod common;
pub mod config;

use common::log::*;
use tracing::info;

fn main() {
    println!("Hello, world!");

    Logger::init(Some("app".to_string()));
    info!("came proxy starting...");
}
