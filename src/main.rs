use std::fmt;

use clap::{Parser, ValueEnum};
use log::warn;
use market_discoverer::config::Config;
use market_discoverer::service::Service;
use market_discoverer::services::coingecko::service::Coingecko;
use market_discoverer::services::ethereum_list::service::EthereumList;
use market_discoverer::services::geckoterminal::service::Geckoterminal;
use market_discoverer::services::layerzero::service::LayerZero;
use market_discoverer::services::stargate_api::service::StargateAPI;

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Args {
    service_name: ServiceName,

    #[arg(long, short = 'u')]
    data_url: Option<String>,

    #[arg(long, short = 'c')]
    cache_file_path: Option<String>,
}

#[derive(Debug, ValueEnum, Clone)]
enum ServiceName {
    EthereumList,
    StargateAPI,
    Geckoterminal,
    Coingecko,
    LayerZero,
}

impl fmt::Display for ServiceName {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // Reuse clap's canonical value string (e.g. `stargate-api`),
        // so logging/paths stay in sync with CLI parsing.
        if let Some(pv) = self.to_possible_value() {
            write!(f, "{}", pv.get_name())
        } else {
            // Should be unreachable with `#[derive(ValueEnum)]`.
            write!(f, "<unknown>")
        }
    }
}

fn main() {
    env_logger::init();

    let args = Args::parse();

    let service_name = args.service_name.to_string();

    if args.cache_file_path.is_none() {
        warn!("Cache file path is not provided, using default path: cache/{service_name}.json");
        return;
    }

    let config = Config {
        name: service_name,
        cache_file_path: args.cache_file_path.unwrap(),
        data_url: args.data_url,
    };

    match args.service_name {
        ServiceName::EthereumList => EthereumList.update_known_entries(config),
        ServiceName::StargateAPI => StargateAPI.update_known_entries(config),
        ServiceName::Geckoterminal => Geckoterminal.update_known_entries(config),
        ServiceName::Coingecko => Coingecko.update_known_entries(config),
        ServiceName::LayerZero => LayerZero.update_known_entries(config),
    };
}
