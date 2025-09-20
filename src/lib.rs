use std::collections::HashMap;
use std::fmt;
use axum::Json;
use reqwest::Client;
use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Clone)]
pub struct AppState {
    pub client: Client,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct ValueIntraday {
    pub value: Value,
}

pub struct PrettyJson<T>(Json<T>);

impl<T> fmt::Display for PrettyJson<T>
where
    T: Serialize,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // Serializamos a string bonito
        match serde_json::to_string_pretty(&self.0 .0) {
            Ok(json_str) => write!(f, "{}", json_str),
            Err(_) => write!(f, "<error serializing JSON>"),
        }
    }
}

pub fn build_url_with_params(base_url: &str, params: HashMap<String, String>) -> String {
    if params.is_empty() {
        base_url.to_string()
    } else {
        let query: Vec<String> = params.iter().map(|(k, v)| format!("{}={}", k, v)).collect();
        format!("{}?{}", base_url, query.join("&"))
    }
}
