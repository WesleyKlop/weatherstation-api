use actix_web::{App, HttpServer};
use weatherstation_api::config::bind_address;
use weatherstation_api::database::create_pool;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv::dotenv().ok();

    HttpServer::new(|| App::new().data(create_pool()))
        .bind(bind_address())?
        .run()
        .await
}
