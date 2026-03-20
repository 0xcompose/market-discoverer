use reqwest::Url;

use crate::{
    service::Service,
    services::ethereum_list::types::{Chain, EthereumListResponseData},
};

pub struct EthereumList;

const ETHEREUM_LIST_ENDPOINT_URL: &str = "https://chainid.network/chains.json";

impl Service for EthereumList {
    type Entry = Chain;
    type ResponseData = EthereumListResponseData;

    fn name() -> &'static str {
        "Ethereum List"
    }

    fn get_default_data_endpoint_url() -> Url {
        Url::parse(ETHEREUM_LIST_ENDPOINT_URL)
            .expect("Invalid default URL for service Ethereum List")
    }
}
