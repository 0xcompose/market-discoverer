use reqwest::Url;

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

    fn get_default_data_endpoint_url() -> Url {
        Url::parse("https://metadata.layerzero-api.com/v1/metadata/deployments")
            .expect("Invalid default URL for service LayerZero Networks")
    }

    fn name() -> &'static str {
        "LayerZero Networks"
    }

    fn fetch_entries(config: &Config) -> Result<Vec<LayerZeroNetworkMetadata>, reqwest::Error> {
        let data_url: Url;

        if let Some(provided_data_url) = &config.data_url {
            data_url = Url::parse(provided_data_url).expect(&format!(
                "Provided invalid URL for service {}",
                Self::name()
            ));
        } else {
            data_url = Self::get_default_data_endpoint_url()
        }

        let client = reqwest::blocking::Client::builder()
            .user_agent(USER_AGENT)
            .build()?;

        let response = client.get(data_url).send()?.error_for_status()?;

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
