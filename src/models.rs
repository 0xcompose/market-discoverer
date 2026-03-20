use crate::{schema::ethereum_list_chains, services::ethereum_list::types::Chain};
use diesel::prelude::*;

#[derive(Insertable, Debug, Queryable)]
#[table_name = "ethereum_list_chains"]
pub struct NewChain {
    pub id: i32,
    pub chain_id: i32,
    pub chain_name: String,
    pub chain_type: String,
    pub chain_status: String,
    pub chain_layer: String,
    pub chain_stack: String,
    pub chain_native_currency: String,
    pub chain_native_currency_address: String,
    pub chain_native_currency_decimals: i32,
    pub chain_native_currency_symbol: String,
    pub chain_native_currency_name: String,
}

impl From<Chain> for NewChain {
    fn from(chain: Chain) -> Self {
        NewChain {
            id: chain.chain_id as i32,
            ..chain.into()
        }
    }
}
