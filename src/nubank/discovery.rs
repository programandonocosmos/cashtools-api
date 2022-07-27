use reqwest;
use serde_json::{self, Value};

#[derive(Debug)]
pub enum DiscoveryError {
    DiscoveryRequestFailed(reqwest::Error),
    DiscoveryRequestDecodingFailed(reqwest::Error),
    DiscoveryJsonConversionFailed(serde_json::Error),
    UrlNameNotFoundInDiscoveryJson(String),
}

pub async fn get_url(name: String) -> Result<String, DiscoveryError> {
    let discovery_app_url = "https://prod-s0-webapp-proxy.nubank.com.br/api/app/discovery";

    let urls_str = reqwest::get(discovery_app_url)
        .await
        .map_err(DiscoveryError::DiscoveryRequestFailed)?
        .text()
        .await
        .map_err(DiscoveryError::DiscoveryRequestDecodingFailed)?;

    let urls = serde_json::from_str::<Value>(&*urls_str)
        .map_err(DiscoveryError::DiscoveryJsonConversionFailed)?;

    match urls.get(&name) {
        Some(v) => Ok(v.to_string().replace("\"", "")),
        _ => Err(DiscoveryError::UrlNameNotFoundInDiscoveryJson(name)),
    }
}
