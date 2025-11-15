use std::env;

use log::{error, info};
use market_discoverer::config::read_config;
use market_discoverer::types::ethereum_list::types::EthereumListResponseData;

fn main() {
    env_logger::init();

    let args: Vec<String> = env::args().collect();

    info!("args: {:?}", args);

    if args.len() > 2 {
        error!("Usage: discover <config_file.toml>");
        return;
    }

    let config_path = &args[1];

    let config = read_config(config_path).unwrap();

    info!("config: {:?}", config);

    market_discoverer::process::update_known_entries::<EthereumListResponseData>(config.clone());
}
