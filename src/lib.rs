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

#[derive(Deserialize)]
pub struct Params {
    function: String,
    symbol: String,
    interval: String,
    apikey: String,
    datatype: String,
}

impl Params {
    pub fn to_hashmap(&self) -> HashMap<String, String> {
        let mut map = HashMap::new();
        map.insert("function".to_string(), self.function.clone());
        map.insert("symbol".to_string(), self.symbol.to_string());
        map.insert("interval".to_string(), self.interval.to_string());
        map.insert("apikey".to_string(), self.apikey.to_string());
        map.insert("datatype".to_string(), self.datatype.to_string());
        map
    }
}

impl fmt::Display for Params {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // Aquí defines cómo quieres que se vea al imprimirlo
        write!(f, "function: {}, symbol: {}, interval: {}, datatype: {}.", self.function, self.symbol, self.interval, self.datatype)
    }
}

#[derive(Serialize)]
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
