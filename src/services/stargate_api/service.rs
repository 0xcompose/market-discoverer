use reqwest::Url;

use crate::{
    service::Service,
    services::stargate_api::types::{Chain, StargateChainsResponseData},
};

pub struct StargateAPI;

impl Service for StargateAPI {
    type Entry = Chain;
    type ResponseData = StargateChainsResponseData;

    fn get_default_data_endpoint_url() -> Url {
        Url::parse("https://transfer.layerzero-api.com/v1/chains")
            .expect("Invalid default URL for service Stargate Chains")
    }

    fn name() -> &'static str {
        "Stargate Chains"
    }
}
