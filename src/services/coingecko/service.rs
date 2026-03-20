use reqwest::Url;

use crate::{
    config::Config,
    service::Service,
    services::{
        coingecko::types::{CoingeckoAssetPlatform, CoingeckoAssetPlatforms},
        traits::ResponseData,
    },
};

pub struct Coingecko;

const API_KEY_HEADER: &str = "X-CG-Pro-API-Key";
const ASSET_PLATFORM_ENDPOINT_URL: &str = "https://api.coingecko.com/api/v3/asset_platforms";

impl Service for Coingecko {
    type Entry = CoingeckoAssetPlatform;
    type ResponseData = CoingeckoAssetPlatforms;

    fn get_default_data_endpoint_url() -> Url {
        Url::parse(ASSET_PLATFORM_ENDPOINT_URL)
            .expect("Invalid default URL for service Coingecko Asset Platforms")
    }

    fn name() -> &'static str {
        "Coingecko Asset Platforms"
    }

    fn fetch_entries(config: &Config) -> Result<Vec<CoingeckoAssetPlatform>, reqwest::Error> {
        dotenv::dotenv().ok();

        let data_url: Url;

        if let Some(provided_data_url) = &config.data_url {
            data_url = Url::parse(provided_data_url).expect(&format!(
                "Provided invalid URL for service {}",
                Self::name()
            ));
        } else {
            data_url = Self::get_default_data_endpoint_url()
        }

        // Retrieve Coingecko API key from environment variable
        let api_key = std::env::var("COINGECKO_API_KEY").expect("COINGECKO_API_KEY is not set");

        let client = reqwest::blocking::Client::new();

        let response = client
            .get(data_url)
            .header(API_KEY_HEADER, api_key)
            .send()?
            .error_for_status()?;

        let data: CoingeckoAssetPlatforms = response.json()?;
        Ok(data.entries())
    }
}
