use crate::services::coingecko::types::{CoingeckoAssetPlatform, CoingeckoAssetPlatforms};
use crate::services::traits::{Entry, EntryId, ResponseData};

impl ResponseData for CoingeckoAssetPlatforms {
    type Entry = CoingeckoAssetPlatform;

    fn entries(&self) -> Vec<Self::Entry> {
        self.clone()
    }
}

impl Entry for CoingeckoAssetPlatform {
    fn id(&self) -> EntryId {
        self.id.clone()
    }

    fn name(&self) -> String {
        self.name.clone()
    }

    fn format(&self) -> String {
        format!(
            "📛 Name: {}\n\
             🔗 Chain ID: {}\n\
             💰 Native Coin: {}",
            self.name,
            self.chain_identifier.unwrap_or_default(),
            self.native_coin_id
        )
    }
}
