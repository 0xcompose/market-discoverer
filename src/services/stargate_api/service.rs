use crate::{
    service::Service,
    services::stargate_api::types::{Chain, StargateChainsResponseData},
};

pub struct StargateAPI;

impl Service for StargateAPI {
    type Entry = Chain;
    type ResponseData = StargateChainsResponseData;

    fn get_data_endpoint_url() -> &'static str {
        "https://stargate.finance/api/v1/chains"
    }

    fn name() -> &'static str {
        "Stargate Chains"
    }
}
