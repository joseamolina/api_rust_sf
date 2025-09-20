use axum::extract::Query;
use axum::{Json, Router, extract::State, response::IntoResponse, routing::get};
use reqwest::Client;
use std::{collections::HashMap, net::SocketAddr};
use serde_json::Value;

#[tokio::main]
async fn main() {
    let state = api_rust_sf::AppState {
        client: Client::new(),
    };

    let app = Router::new()
        .route("/get_value_intraday", get(get_value_intraday))
        .with_state(state);

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));

    println!("listening on {}", addr);
    axum::serve(tokio::net::TcpListener::bind(addr).await.unwrap(), app)
        .await
        .unwrap();
}

async fn get_value_intraday(
    State(state): State<api_rust_sf::AppState>,
    Query(params): Query<HashMap<String, String>>,
) -> impl IntoResponse {
    let base_url = "https://alphavantage.co/query";
    let url = api_rust_sf::build_url_with_params(base_url, params);

    match state.client.get(url).send().await {
        Ok(resp) => {
            let text = resp.text().await.unwrap_or_default();
            let value_json: Value = serde_json::from_str(&text).unwrap();
            Json(api_rust_sf::ValueIntraday { value: value_json })
        }
        Err(_) => {
            Json(api_rust_sf::ValueIntraday { value: Value::Null })
        }
    }
}
