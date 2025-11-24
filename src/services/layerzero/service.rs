use crate::{
    config::Config,
    service::Service,
    services::{
        layerzero::types::{
            ChainStatus, LayerZeroNetworkMetadata, LayerZeroNetworksMetadata, Stage,
        },
        traits::ResponseData,
    },
};

pub struct LayerZero;

const USER_AGENT: &str = "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/120.0.0.0 Safari/537.36";

impl Service for LayerZero {
    type Entry = LayerZeroNetworkMetadata;
    type ResponseData = LayerZeroNetworksMetadata;

    fn fetch_entries(
        &self,
        config: &Config,
    ) -> Result<Vec<LayerZeroNetworkMetadata>, reqwest::Error> {
        let client = reqwest::blocking::Client::builder()
            .user_agent(USER_AGENT)
            .build()?;

        let response = client
            .get(&config.data_endpoint_url)
            .send()?
            .error_for_status()?;

        let data: LayerZeroNetworksMetadata = response.json()?;

        Ok(data.entries())
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
