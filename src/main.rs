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
        "Ethereum List" => EthereumList.process(config),
        "Stargate Chains" => StargateAPI.process(config),
        "Geckoterminal Networks" => Geckoterminal.process(config),
        "Coingecko Asset Platforms" => Coingecko.process(config),
        "LayerZero Networks" => LayerZero.process(config),
        _ => {
            error!("Invalid service name: {}", config.name);
            return;
        }
    };
}
