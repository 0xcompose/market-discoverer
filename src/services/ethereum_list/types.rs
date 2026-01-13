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
    pub name: String,
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
