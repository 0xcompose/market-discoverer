use log::debug;

use crate::{
    config::Config,
    service::Service,
    services::{
        geckoterminal::types::{GeckoterminalNetworksData, Network},
        traits::ResponseData,
    },
};

pub struct Geckoterminal;

impl Service for Geckoterminal {
    type Entry = Network;
    type ResponseData = GeckoterminalNetworksData;

    // Fetches Networks from Geckoterminal API iterating through all pages
    fn fetch_entries(&self, config: &Config) -> Result<Vec<Network>, reqwest::Error> {
        let mut entries = Vec::new();
        let mut next_page_url: Option<String> = Some(config.data_endpoint_url.clone());

        while let Some(url) = next_page_url {
            debug!("Fetching entries from {}", url);
            let response = reqwest::blocking::get(url)?.error_for_status()?;
            let data: GeckoterminalNetworksData = response.json()?;

            entries.extend(data.entries());

            next_page_url = data
                .links
                .next
                .and_then(|v| v.as_str().map(|s| s.to_string()));
        }

        Ok(entries)
    }
}
