mod api;
mod config;
mod models;
mod service;
use actix_web::{web, App, HttpServer};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let app_config = config::AppConfig::init();

    println!("{}", app_config.app_url);
    HttpServer::new(move || {
        App::new()
            .route("/health", web::get().to(api::health_check))
            .route(
                "/api/v1/get_block_number",
                web::get().to(api::get_block_number),
            )
            .route(
                "/api/v1/get_transactions",
                web::post().to(api::get_transactions),
            )
    })
    .bind(app_config.app_url)?
    .run()
    .await
}
