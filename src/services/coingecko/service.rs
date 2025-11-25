use crate::{
    service::Service,
    services::coingecko::types::{CoingeckoAssetPlatform, CoingeckoAssetPlatforms},
    services::traits::ResponseData,
};

pub struct Coingecko;

const API_KEY_HEADER: &str = "X-CG-Pro-API-Key";
const ASSET_PLATFORM_ENDPOINT_URL: &str = "https://api.coingecko.com/api/v3/asset_platforms";

impl Service for Coingecko {
    type Entry = CoingeckoAssetPlatform;
    type ResponseData = CoingeckoAssetPlatforms;

    fn get_data_endpoint_url() -> &'static str {
        ASSET_PLATFORM_ENDPOINT_URL
    }

    fn name() -> &'static str {
        "Coingecko Asset Platforms"
    }

    fn fetch_entries() -> Result<Vec<CoingeckoAssetPlatform>, reqwest::Error> {
        dotenv::dotenv().ok();

        // Retrieve Coingecko API key from environment variable
        let api_key = std::env::var("COINGECKO_API_KEY").expect("COINGECKO_API_KEY is not set");

        let client = reqwest::blocking::Client::new();

        let response = client
            .get(ASSET_PLATFORM_ENDPOINT_URL)
            .header(API_KEY_HEADER, api_key)
            .send()?
            .error_for_status()?;

        let data: CoingeckoAssetPlatforms = response.json()?;
        Ok(data.entries())
    }
}
