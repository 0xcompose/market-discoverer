// Example code that deserializes and serializes the model.
// extern crate serde;
// #[macro_use]
// extern crate serde_derive;
// extern crate serde_json;
//
// use generated_module::CoingeckoAssetPlatforms;
//
// fn main() {
//     let json = r#"{"answer": 42}"#;
//     let model: CoingeckoAssetPlatforms = serde_json::from_str(&json).unwrap();
// }

use serde::{Deserialize, Serialize};

pub type CoingeckoAssetPlatforms = Vec<CoingeckoAssetPlatform>;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CoingeckoAssetPlatform {
    pub id: String,

    pub chain_identifier: Option<i64>,

    pub name: String,

    pub shortname: String,

    pub native_coin_id: String,

    pub image: Image,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Image {
    pub thumb: Option<String>,

    pub small: Option<String>,

    pub large: Option<String>,
}
