use gateway::Gateway;

pub mod auth;
pub mod common;
pub mod config;
pub mod gateway;
pub mod health;
pub mod limits;
pub mod load_balance;
pub mod protocols;

fn main() {
    let gateway = Gateway::new("127.0.0.1".to_string(), 8080);
    gateway.run();
}
