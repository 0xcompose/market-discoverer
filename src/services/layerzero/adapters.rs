use log::debug;

use crate::services::{
    layerzero::types::{LayerZeroNetworkMetadata, LayerZeroNetworksMetadata},
    traits::{Entry, EntryId, ResponseData},
};

fn opt_display<T>(opt: &Option<T>, default: &str) -> String
where
    T: std::fmt::Display,
{
    opt.as_ref()
        .map(|v| v.to_string())
        .unwrap_or_else(|| default.to_string())
}

fn opt_debug<T>(opt: &Option<T>, default: &str) -> String
where
    T: std::fmt::Debug,
{
    opt.as_ref()
        .map(|v: &T| format!("{:?}", v))
        .unwrap_or_else(|| default.to_string())
}

impl ResponseData for LayerZeroNetworksMetadata {
    type Entry = LayerZeroNetworkMetadata;

    fn entries(&self) -> Vec<Self::Entry> {
        self.values().cloned().collect()
    }
}

impl Entry for LayerZeroNetworkMetadata {
    fn id(&self) -> EntryId {
        self.chain_key.clone()
    }

    fn name(&self) -> String {
        self.chain_key.clone()
    }

    fn format(&self) -> String {
        let Some(chain_details) = &self.chain_details else {
            return format!("📛 Name: {} (No chain details)", self.chain_key);
        };

        let native_currency_str = chain_details
            .native_currency
            .as_ref()
            .map(|nc| format!("{} ({})", nc.symbol, opt_display(&nc.decimals, "N/A")))
            .unwrap_or_else(|| "N/A".to_string());

        format!(
            "📛 Name: {}\n\
            Chain ID: {}\n\
            Type: {}\n\
            Status: {}\n\
            Layer: {}\n\
            Stack: {}\n\
            Native Currency: {}\n\
            Average Block Time: {}\n\
            Mainnet Chain Name: {}\n\
            CoinGecko Network ID: {}\n\n\n",
            // Explorer: {}",
            self.chain_key,
            opt_display(&chain_details.native_chain_id, "N/A"),
            chain_details.chain_type,
            opt_debug(&chain_details.chain_status, "N/A"),
            opt_debug(&chain_details.chain_layer, "N/A"),
            opt_debug(&chain_details.chain_stack, "N/A"),
            native_currency_str,
            opt_display(&chain_details.average_block_time, "N/A"),
            chain_details.mainnet_chain_name.as_deref().unwrap_or("N/A"),
            chain_details.cg_network_id.as_deref().unwrap_or("N/A"),
            // self.block_explorers
        )
    }
}
