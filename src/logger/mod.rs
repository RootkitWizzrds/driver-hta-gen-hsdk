use log::{info, LevelFilter};
use env_logger::Builder;
use std::env;

pub fn init_logger() {
    let env = env_logger::Env::default().filter_or("LOG_LEVEL", "info");
    Builder::from_env(env).init();
    info!("[@] Logger initialized");
}
