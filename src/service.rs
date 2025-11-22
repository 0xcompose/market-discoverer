use crate::{
    clients::{
        ethereum_list::types::EthereumListResponseData,
        geckoterminal::types::GeckoterminalNetworksData,
        stargate_api::types::StargateChainsResponseData,
    },
    config::Config,
    process::update_known_entries,
};

#[derive(Debug, Clone, Copy)]
pub enum Service {
    EthereumList,
    StargateChains,
    GeckoterminalNetworks,
}

impl Service {
    pub fn from_config_name(name: &str) -> Option<Self> {
        match name {
            "Ethereum List" => Some(Service::EthereumList),
            "Stargate Chains" => Some(Service::StargateChains),
            "Geckoterminal Networks" => Some(Service::GeckoterminalNetworks),
            _ => None,
        }
    }

    pub fn process(&self, config: Config) {
        match self {
            Service::EthereumList => {
                update_known_entries::<EthereumListResponseData>(config);
            }
            Service::StargateChains => {
                update_known_entries::<StargateChainsResponseData>(config);
            }
            Service::GeckoterminalNetworks => {
                update_known_entries::<GeckoterminalNetworksData>(config);
            }
        }
    }
}
