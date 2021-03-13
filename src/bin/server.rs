use actix_web::{middleware, web::scope, App, HttpServer};
use weatherstation_api::config::bind_address;
use weatherstation_api::database::create_pool;
use weatherstation_api::routes;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv::dotenv().ok();

    HttpServer::new(|| {
        App::new()
            .wrap(middleware::NormalizePath::default())
            .data(create_pool())
            .service(
                scope("/api/measurements")
                    .service(routes::all_measurements)
                    .service(routes::measurement_by_id),
            )
    })
    .bind(bind_address())?
    .run()
    .await
}
