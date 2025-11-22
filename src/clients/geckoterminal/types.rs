// Example code that deserializes and serializes the model.
// extern crate serde;
// #[macro_use]
// extern crate serde_derive;
// extern crate serde_json;
//
// use generated_module::GeckoterminalNetworksData;
//
// fn main() {
//     let json = r#"{"answer": 42}"#;
//     let model: GeckoterminalNetworksData = serde_json::from_str(&json).unwrap();
// }

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GeckoterminalNetworksData {
    pub data: Vec<Network>,

    pub links: Links,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Network {
    pub id: String,

    #[serde(rename = "type")]
    pub network_type: NetworkType,

    pub attributes: Attributes,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Attributes {
    pub name: String,

    pub coingecko_asset_platform_id: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum NetworkType {
    Network,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Links {
    pub first: String,

    pub prev: Option<serde_json::Value>,

    pub next: String,

    pub last: String,
}
