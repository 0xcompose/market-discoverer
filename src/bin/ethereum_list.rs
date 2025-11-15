use log::info;
use market_discoverer::config::Config;
use market_discoverer::types::ethereum_list::types::EthereumListResponseData;

const SERVICE_CONFIG: Config = Config {
    name: "Ethereum List",
    data_endpoint_url: "https://chainid.network/chains.json",
    cache_file_path: "cache/ethereum_lists.json",
};

fn main() {
    env_logger::init();
    info!("SERVICE_CONFIG: {:?}", SERVICE_CONFIG);

    market_discoverer::process::update_known_entries::<EthereumListResponseData>(SERVICE_CONFIG);
}
