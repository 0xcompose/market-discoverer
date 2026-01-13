// Example code that deserializes and serializes the model.
// extern crate serde;
// #[macro_use]
// extern crate serde_derive;
// extern crate serde_json;
//
// use generated_module::TopLevel;
//
// fn main() {
//     let json = r#"{"answer": 42}"#;
//     let model: TopLevel = serde_json::from_str(&json).unwrap();
// }

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct StargateChainsResponseData {
    pub chains: Vec<Chain>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Chain {
    pub chain_key: String,

    pub chain_type: String,

    pub chain_id: i64,

    pub short_name: String,

    pub name: String,

    pub native_currency: NativeCurrency,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct NativeCurrency {
    pub chain_key: String,

    pub name: String,

    pub symbol: String,

    pub decimals: i64,

    pub address: String,
}
