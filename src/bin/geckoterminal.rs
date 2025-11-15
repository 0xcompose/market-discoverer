use log::info;
use market_discoverer::config::Config;
use market_discoverer::types::geckoterminal::types::GeckoterminalNetworksData;

const SERVICE_CONFIG: Config = Config {
    name: "Geckoterminal Networks",
    data_endpoint_url: "https://api.geckoterminal.com/api/v2/networks?page=1",
    cache_file_path: "cache/geckoterminal_networks.json",
};

fn main() {
    env_logger::init();
    info!("SERVICE_CONFIG: {:?}", SERVICE_CONFIG);

    market_discoverer::process::update_known_entries::<GeckoterminalNetworksData>(SERVICE_CONFIG);
}
