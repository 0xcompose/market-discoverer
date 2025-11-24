use serde::{Deserialize, Serialize};
use std::collections::HashMap;

pub type LayerZeroNetworksMetadata = HashMap<String, LayerZeroNetworkMetadata>;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LayerZeroNetworkMetadata {
    pub chain_key: String,

    pub chain_details: Option<ChainDetails>,

    pub deployments: Option<Vec<Deployment>>,

    pub dvns: Option<HashMap<String, Dvn>>,

    pub block_explorers: Option<Vec<BlockExplorer>>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ChainDetails {
    pub chain_type: String,

    pub chain_key: String,

    #[serde(rename = "chainStatus")]
    pub chain_status: Option<ChainStatus>,

    #[serde(rename = "chainLayer")]
    pub chain_layer: Option<ChainLayer>,

    pub native_currency: Option<NativeCurrency>,

    pub native_chain_id: Option<i64>,

    pub chain_stack: Option<ChainStack>,

    pub average_block_time: Option<i64>,

    pub mainnet_chain_name: Option<String>,

    pub cg_network_id: Option<String>,

    pub name: Option<String>,

    pub short_name: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum ChainStatus {
    #[serde(rename = "ACTIVE")]
    Active,

    #[serde(rename = "DEPRECATED")]
    Deprecated,

    #[serde(rename = "PRIVATE")]
    Private,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum ChainLayer {
    #[serde(rename = "L1")]
    L1,

    #[serde(rename = "L2")]
    L2,

    #[serde(rename = "L3")]
    L3,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum ChainStack {
    #[serde(rename = "ARB_STACK")]
    ArbStack,

    #[serde(rename = "AVALANCHE_STACK")]
    AvalancheStack,

    #[serde(rename = "OP_STACK")]
    OpStack,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct NativeCurrency {
    pub name: Option<String>,

    pub symbol: String,

    pub cg_id: Option<String>,

    pub cmc_id: Option<i64>,

    pub address: Option<String>,

    pub decimals: Option<i64>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Deployment {
    pub eid: Option<String>,

    pub chain_key: Option<String>,

    pub stage: Option<Stage>,

    pub version: Option<i64>,

    pub endpoint: Option<Endpoint>,

    pub endpoint_v2: Option<Endpoint>,

    pub endpoint_v2_view: Option<Endpoint>,

    pub relayer: Option<Endpoint>,

    pub relayer_v2: Option<Endpoint>,

    pub f_p_validator: Option<Endpoint>,

    pub treasury_v2: Option<Endpoint>,

    pub ultra_light_node: Option<Endpoint>,

    pub ultra_light_node_v2: Option<Endpoint>,

    // Some fields can be either Endpoint objects or strings
    pub send_uln301: Option<serde_json::Value>,

    pub receive_uln301: Option<serde_json::Value>,

    pub send_uln302: Option<Endpoint>,

    pub receive_uln302: Option<Endpoint>,

    pub nonce_contract: Option<Endpoint>,

    pub executor: Option<Endpoint>,

    pub lz_executor: Option<Endpoint>,

    pub blocked_message_lib: Option<Endpoint>,

    #[serde(rename = "mPTValidator01")]
    pub m_pt_validator01: Option<Endpoint>,

    #[serde(rename = "mPTValidatorV4")]
    pub m_pt_validator_v4: Option<Endpoint>,

    #[serde(rename = "mPTValidatorV5")]
    pub m_pt_validator_v5: Option<Endpoint>,

    #[serde(rename = "deadDVN")]
    pub dead_dvn: Option<Endpoint>,

    pub controller: Option<Endpoint>,

    pub dvn_proxy: Option<Endpoint>,

    pub uln_manager: Option<Endpoint>,

    pub executor_proxy: Option<Endpoint>,

    pub all_storages: Option<Endpoint>,

    pub sml_manager: Option<Endpoint>,

    pub pricefeed: Option<Endpoint>,

    pub dvn: Option<Endpoint>,

    pub read_lib1002: Option<Endpoint>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Endpoint {
    pub address: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Stage {
    #[serde(rename = "mainnet")]
    Mainnet,

    #[serde(rename = "sandbox")]
    Sandbox,

    #[serde(rename = "testnet")]
    Testnet,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Dvn {
    pub version: i64,

    pub canonical_name: String,

    pub id: Option<String>,

    pub deprecated: Option<bool>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BlockExplorer {
    pub url: String,
}
