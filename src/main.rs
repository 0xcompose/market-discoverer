use std::env;

use log::{error, info};
use market_discoverer::config::read_config;
use market_discoverer::service::Service;
use market_discoverer::services::coingecko::service::Coingecko;
use market_discoverer::services::ethereum_list::service::EthereumList;
use market_discoverer::services::geckoterminal::service::Geckoterminal;
use market_discoverer::services::layerzero::service::LayerZero;
use market_discoverer::services::stargate_api::service::StargateAPI;

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

    match &*config.name {
        "Ethereum List" => EthereumList.update_known_entries(config),
        "Stargate Chains" => StargateAPI.update_known_entries(config),
        "Geckoterminal Networks" => Geckoterminal.update_known_entries(config),
        "Coingecko Asset Platforms" => Coingecko.update_known_entries(config),
        "LayerZero Networks" => LayerZero.update_known_entries(config),
        _ => {
            error!("Invalid service name: {}", config.name);
            return;
        }
    };
}
