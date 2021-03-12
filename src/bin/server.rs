use actix_web::{App, HttpServer};
use std::error::Error;
use weatherstation_api::config::bind_address;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv::dotenv().ok();

    HttpServer::new(|| App::new())
        .bind(bind_address())?
        .run()
        .await
}
