use log::debug;
use reqwest::Url;

use crate::{
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

    fn get_data_endpoint_url() -> &'static str {
        "https://api.geckoterminal.com/api/v2/networks"
    }

    fn name() -> &'static str {
        "Geckoterminal Networks"
    }

    // Fetches Networks from Geckoterminal API iterating through all pages
    fn fetch_entries() -> Result<Vec<Network>, reqwest::Error> {
        let mut base_url = Url::parse(Self::get_data_endpoint_url())
            .expect(&format!("Invalid URL for service {}", Self::name()));

        base_url.set_query(Some("page=1"));

        let mut entries = Vec::new();
        let mut next_page_url: Option<String> = Some(base_url.to_string());

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
