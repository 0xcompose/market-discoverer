use crate::{
    service::Service,
    services::stargate_api::types::{Chain, StargateChainsResponseData},
};

pub struct StargateAPI;

impl Service for StargateAPI {
    type Entry = Chain;
    type ResponseData = StargateChainsResponseData;
}
