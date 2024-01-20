use std::u16;

use tracing::info;

use crate::common::log::Logger;

pub struct Gateway {
    lan_ip: String,
    lan_port: u16,
}

impl Gateway {
    pub fn new(ip: String, port: u16) -> Self {
        Self {
            lan_ip: ip,
            lan_port: port,
        }
    }

    pub fn run(&self) {
        Logger::init(None);
        info!(
            "server [ip: {}, porrt: {}] starting...",
            self.lan_ip, self.lan_port
        );
    }
}
