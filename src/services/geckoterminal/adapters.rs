use crate::services::geckoterminal::types::{GeckoterminalNetworksData, Network};
use crate::services::traits::{Entry, EntryId, ResponseData};

impl ResponseData for GeckoterminalNetworksData {
    type Entry = Network;

    fn entries(&self) -> Vec<Self::Entry> {
        self.data.clone()
    }
}

impl Entry for Network {
    fn id(&self) -> EntryId {
        self.id.clone()
    }

    fn name(&self) -> String {
        self.attributes.name.clone()
    }

    fn format(&self) -> String {
        format!(
            "📛 Name: {}\n\
             🔗 Chain ID: {}",
            self.name(),
            self.id()
        )
    }
}
