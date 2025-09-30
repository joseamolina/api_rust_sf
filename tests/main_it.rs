use axum::{Json};
use reqwest::Client;
use std::{collections::HashMap};
use serde_json::Value;
use std::fs;

// This method retrieves a key from a dict as a json file
fn get_property_api_key() -> String {
    let content_api = fs::read_to_string("config.json").expect("File could not be read");
    let config: HashMap<String, String> = serde_json::from_str(&content_api).unwrap();
    let api_key = config.get("API").unwrap().clone();
    api_key
}

#[tokio::test]
pub async fn test_joke_endpoint_with_params() {
    let state = api_rust_sf::AppState {
        client: Client::new(),
    };

    let content_api: String = get_property_api_key();

    let mut params = HashMap::new();
    params.insert("function".to_string(), "TIME_SERIES_INTRADAY".to_string());
    params.insert("symbol".to_string(), "MSFT".to_string());
    params.insert("interval".to_string(), "5min".to_string());
    params.insert("apikey".to_string(), content_api.to_string());
    params.insert("datatype".to_string(), "json".to_string());

    // Construimos la URL completa con query params
    let url = reqwest::Url::parse_with_params(
        &format!("{}", "https://alphavantage.co/query"),
        &params,
    ).unwrap();

    let payload = match state.client.get(url).send().await {
        Ok(resp) => {
            let text = resp.text().await.unwrap_or_default();
            let joke: Value = serde_json::from_str(&text).unwrap();
            Json(api_rust_sf::ValueIntraday { value: joke })
        }
        Err(_) => {
            Json(api_rust_sf::ValueIntraday { value: Value::Null })
        }
    };

    assert!(!payload.value["Time Series (5min)"].is_null());
}