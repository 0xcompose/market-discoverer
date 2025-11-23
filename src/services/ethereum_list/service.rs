use crate::{
    service::Service,
    services::ethereum_list::types::{Chain, EthereumListResponseData},
};

pub struct EthereumList;

impl Service for EthereumList {
    type Entry = Chain;
    type ResponseData = EthereumListResponseData;
}
