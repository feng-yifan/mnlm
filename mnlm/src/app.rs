mod config;
mod socket_manager;

pub use config::*;

pub struct App {
    config: Config,
}
