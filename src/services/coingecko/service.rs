use crate::{
    config::Config,
    service::Service,
    services::coingecko::types::{CoingeckoAssetPlatform, CoingeckoAssetPlatforms},
    services::traits::ResponseData,
};

pub struct Coingecko;

const API_KEY_HEADER: &str = "X-CG-Pro-API-Key";

impl Service for Coingecko {
    type Entry = CoingeckoAssetPlatform;
    type ResponseData = CoingeckoAssetPlatforms;

    fn fetch_entries(
        &self,
        config: &Config,
    ) -> Result<Vec<CoingeckoAssetPlatform>, reqwest::Error> {
        dotenv::dotenv().ok();

        // Retrieve Coingecko API key from environment variable
        let api_key = std::env::var("COINGECKO_API_KEY").expect("COINGECKO_API_KEY is not set");

        let client = reqwest::blocking::Client::new();

        let response = client
            .get(&config.data_endpoint_url)
            .header(API_KEY_HEADER, api_key)
            .send()?
            .error_for_status()?;

        let data: CoingeckoAssetPlatforms = response.json()?;
        Ok(data.entries())
    }
}
