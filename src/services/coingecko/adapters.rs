use crate::services::coingecko::types::{CoingeckoAssetPlatform, CoingeckoAssetPlatforms};
use crate::services::traits::{Entry, EntryId, ResponseData};
use crate::utils::opt_display;

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
        opt_display(&self.name, "N/A")
    }

    fn format(&self) -> String {
        format!(
            "📛 Name: {}\n\
             🔗 Chain ID: {}\n\
             💰 Native Coin: {}",
            opt_display(&self.name, "N/A"),
            opt_display(&self.chain_identifier, "N/A"),
            opt_display(&self.native_coin_id, "N/A")
        )
    }
}
