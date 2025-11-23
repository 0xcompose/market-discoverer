// Example code that deserializes and serializes the model.
// extern crate serde;
// #[macro_use]
// extern crate serde_derive;
// extern crate serde_json;
//
// use generated_module::EthereumListResponseData;
//
// fn main() {
//     let json = r#"{"answer": 42}"#;
//     let model: EthereumListResponseData = serde_json::from_str(&json).unwrap();
// }

use serde::{Deserialize, Serialize};

pub type EthereumListResponseData = Vec<Chain>;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Chain {
    pub name: String,

    pub chain: String,

    pub icon: Option<String>,

    pub rpc: Vec<String>,

    pub features: Option<Vec<Feature>>,

    pub faucets: Vec<String>,

    pub native_currency: NativeCurrency,

    #[serde(rename = "infoURL")]
    pub info_url: String,

    pub short_name: String,

    pub chain_id: i64,

    pub network_id: i64,

    pub slip44: Option<i64>,

    pub ens: Option<Ens>,

    pub explorers: Option<Vec<Explorer>>,

    pub title: Option<String>,

    pub status: Option<Status>,

    pub red_flags: Option<Vec<RedFlag>>,

    pub parent: Option<Parent>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Ens {
    pub registry: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Explorer {
    pub name: String,

    pub url: String,

    pub standard: Standard,

    pub icon: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum Standard {
    #[serde(rename = "EIP3091")]
    Eip3091,

    #[serde(rename = "none")]
    None,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Feature {
    pub name: Name,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum Name {
    #[serde(rename = "Custom Gas Model")]
    CustomGasModel,

    #[serde(rename = "Directed Acyclic Graph (DAG)")]
    DirectedAcyclicGraphDag,

    #[serde(rename = "EIP1108")]
    Eip1108,

    #[serde(rename = "EIP155")]
    Eip155,

    #[serde(rename = "EIP1559")]
    Eip1559,

    #[serde(rename = "EIP20")]
    Eip20,

    #[serde(rename = "EIP55")]
    Eip55,

    #[serde(rename = "EIP6551")]
    Eip6551,

    #[serde(rename = "Low-Latency Transactions")]
    LowLatencyTransactions,

    #[serde(rename = "none")]
    None,

    #[serde(rename = "Smart Contracts")]
    SmartContracts,

    #[serde(rename = "Unspent Transaction Output (UTXO)")]
    UnspentTransactionOutputUtxo,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct NativeCurrency {
    pub name: String,

    pub symbol: String,

    pub decimals: i64,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Parent {
    #[serde(rename = "type")]
    pub parent_type: Type,

    pub chain: String,

    pub bridges: Option<Vec<Bridge>>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Bridge {
    pub url: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum Type {
    L2,

    #[serde(rename = "shard")]
    Shard,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum RedFlag {
    #[serde(rename = "reusedChainId")]
    ReusedChainId,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Status {
    Active,

    Deprecated,

    Incubating,
}
