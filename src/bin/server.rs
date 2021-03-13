use actix_web::{App, HttpServer};
use weatherstation_api::config::bind_address;
use weatherstation_api::database::create_pool;
use weatherstation_api::routes;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv::dotenv().ok();

    HttpServer::new(|| {
        App::new()
            .data(create_pool())
            .service(routes::all_measurements)
    })
    .bind(bind_address())?
    .run()
    .await
}
