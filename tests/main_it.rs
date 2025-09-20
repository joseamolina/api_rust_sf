use core::contracts::requires;
use api_rust_sf::{get_value_intraday};
use axum::{Router, routing::get, extract::State, Json};
use reqwest::Client;
use std::{collections::HashMap, net::SocketAddr};
use tokio::task;

async fn spawn_server() -> SocketAddr {
    let state = AppState {
        client: Client::new(),
    };

    let app = Router::new()
        .route(
            "/joke",
            get(|State(state): State<AppState>, Query(params): Query<HashMap<String, String>>| async move {
                let url = build_url_with_params("https://api.chucknorris.io/jokes/random", &params);
                let resp = state.client.get(&url).send().await.unwrap();
                let joke: Joke = resp.json().await.unwrap();
                Json(joke)
            }),
        )
        .with_state(state);

    let addr = SocketAddr::from(([127, 0, 0, 1], 0));
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    let local_addr = listener.local_addr().unwrap();

    // Ejecutamos el servidor en background
    task::spawn(async move {
        axum::serve(listener, app).await.unwrap();
    });

    local_addr
}

#[tokio::test]
pub async fn test_joke_endpoint_with_params() {
    let addr = spawn_server().await;
    let client = reqwest::Client::new();

    // Creamos un HashMap de parámetros para enviar
    let mut params = HashMap::new();
    params.insert("function".to_string(), "TIME_SERIES_INTRADAY".to_string());
    params.insert("symbol".to_string(), "MSFT".to_string());
    params.insert("interval".to_string(), "5min".to_string());
    params.insert("apikey".to_string(), "MVYU2FHVTM8AHW8V".to_string());
    params.insert("datatype".to_string(), "json".to_string());

    // Construimos la URL completa con query params
    let url = reqwest::Url::parse_with_params(
        &format!("http://{}/joke", addr),
        &params,
    ).unwrap();

    let resp = client.get(url).send().await.unwrap();
    assert!(resp.status().is_success());

    let payload: ValueIntraday = resp.json().await.unwrap();

    assert!(!joke.value.is_empty());
}