use crate::clients::common::{Entry, EntryId, ResponseData};
use crate::clients::geckoterminal::types::{GeckoterminalNetworksData, Network};

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
