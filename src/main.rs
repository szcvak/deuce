mod rc4;
mod network;
mod logging;
mod settings;
mod reader;
mod writer;
mod packets;
mod device;
mod database;
mod player;
mod csv_logic;
mod math;
mod checksum;
mod milestones;

use config::Config;
use log::*;

use crate::logging::*;
use crate::network::Network;
use crate::settings::*;

fn main() {
    init_logging();
    let settings = Settings::load("deuce.toml").expect("deuce: failed to load settings");

    info!("started server on 0.0.0.0:{}", settings.port);

    let server = Network::new(settings);
    server.start();
}
