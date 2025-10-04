use actix_web::{get, web, App, HttpServer, Responder};

async fn ext_api_call(url: &str) -> Result<String, reqwest::Error> {
    let resp = reqwest::get(url).await?;
    let body = resp.text().await?;
    Ok(body)
}

#[get("/get_value_intraday")]
async fn get_value_intraday(params: web::Query<api_rust_sf::Params>) -> impl Responder {
    let base_url = "https://alphavantage.co/query";

    println!("Petición");
    let url = api_rust_sf::build_url_with_params(base_url, params.into_inner().to_hashmap());
    let response = ext_api_call(&url).await.unwrap();

    // Calcula la suma
    let resultado = api_rust_sf::ValueIntraday {
        value: response.parse().unwrap()
    };

    web::Json(resultado)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {

    println!("Starting server...");
    HttpServer::new(|| {
        App::new()
            .service(get_value_intraday)
    })
        .bind(("0.0.0.0", 8085))?
        .run()
        .await
}
