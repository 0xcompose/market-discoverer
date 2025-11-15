use log::info;
use market_discoverer::{config::Config, types::stargate_api::types::StargateChainsResponseData};

const SERVICE_CONFIG: Config = Config {
    name: "Stargate Chains",
    data_endpoint_url: "https://stargate.finance/api/v1/chains",
    cache_file_path: "cache/stargate_chains.json",
};

fn main() {
    env_logger::init();
    info!("SERVICE_CONFIG: {:?}", SERVICE_CONFIG);

    market_discoverer::process::update_known_entries::<StargateChainsResponseData>(SERVICE_CONFIG);
}
