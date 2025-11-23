use crate::services::ethereum_list::types::{Chain, EthereumListResponseData};
use crate::services::traits::{Entry, EntryId, ResponseData};

impl ResponseData for EthereumListResponseData {
    type Entry = Chain;

    fn entries(&self) -> Vec<Self::Entry> {
        self.clone()
    }
}

impl Entry for Chain {
    fn id(&self) -> EntryId {
        self.chain_id.to_string()
    }

    fn name(&self) -> String {
        self.name.clone()
    }

    fn format(&self) -> String {
        format!(
            "📛 Name: {}\n\
             🔗 Chain ID: {}\n\
             💰 Native Currency: {} ({})\n\
             🔢 Decimals: {}",
            self.name,
            self.chain_id,
            self.native_currency.symbol,
            self.native_currency.name,
            self.native_currency.decimals
        )
    }
}
