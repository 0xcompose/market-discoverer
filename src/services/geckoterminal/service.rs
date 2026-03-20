use log::debug;
use reqwest::Url;

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

    fn get_default_data_endpoint_url() -> Url {
        Url::parse("https://api.geckoterminal.com/api/v2/networks")
            .expect("Invalid default URL for service Geckoterminal Networks")
    }

    fn name() -> &'static str {
        "Geckoterminal Networks"
    }

    // Fetches Networks from Geckoterminal API iterating through all pages
    fn fetch_entries(config: &Config) -> Result<Vec<Network>, reqwest::Error> {
        let mut data_url: Url;

        if let Some(provided_data_url) = &config.data_url {
            data_url = Url::parse(provided_data_url).expect(&format!(
                "Provided invalid URL for service {}",
                Self::name()
            ));
        } else {
            data_url = Self::get_default_data_endpoint_url()
        }

        data_url.set_query(Some("page=1"));

        let mut entries = Vec::new();
        let mut next_page_url: Option<String> = Some(data_url.to_string());

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
