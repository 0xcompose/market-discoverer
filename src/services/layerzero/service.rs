use reqwest::Url;

use crate::{
    service::Service,
    services::layerzero::types::{
        ChainStatus, LayerZeroNetworkMetadata, LayerZeroNetworksMetadata, Stage,
    },
};

pub struct LayerZero;

impl Service for LayerZero {
    type Entry = LayerZeroNetworkMetadata;
    type ResponseData = LayerZeroNetworksMetadata;

    fn get_default_data_endpoint_url() -> Url {
        Url::parse("https://metadata.layerzero-api.com/v1/metadata/deployments")
            .expect("Invalid default URL for service LayerZero Networks")
    }

    fn name() -> &'static str {
        "LayerZero Networks"
    }

    fn filter_entries(entries: &[Self::Entry]) -> Vec<Self::Entry> {
        entries
            .iter()
            .filter(|entry| !should_skip(entry))
            .cloned()
            .collect()
    }
}

fn should_skip(entry: &LayerZeroNetworkMetadata) -> bool {
    let Some(chain_details) = &entry.chain_details else {
        return true;
    };

    if chain_details.chain_status == Some(ChainStatus::Deprecated) {
        return true;
    }

    if let Some(deployments) = &entry.deployments {
        if deployments.iter().any(|deployment| {
            deployment.stage == Some(Stage::Testnet) || deployment.stage == Some(Stage::Sandbox)
        }) {
            return true;
        }
    };

    entry.chain_key.contains("testnet")
        || entry.chain_key.contains("sepolia")
        || entry.chain_key.contains("sandbox")
        || entry.chain_key.contains("devnet")
}
