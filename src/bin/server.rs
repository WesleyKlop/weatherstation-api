use actix_web::{middleware, web::scope, App, HttpServer};
use actix_web_httpauth::middleware::HttpAuthentication;
use weatherstation_api::config::bind_address;
use weatherstation_api::database::create_pool;
use weatherstation_api::routes;
use weatherstation_api::utils::validator;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv::dotenv().ok();
    env_logger::init();

    let pool = create_pool();

    HttpServer::new(move || {
        App::new()
            .wrap(middleware::Logger::default())
            .wrap(middleware::NormalizePath::default())
            .data(pool.clone())
            .service(
                scope("/api")
                    .service(
                        scope("/measurements")
                            .wrap(HttpAuthentication::bearer(validator))
                            .service(routes::all_measurements)
                            .service(routes::measurement_by_id)
                            .service(routes::create_measurement),
                    )
                    .service(routes::health),
            )
    })
    .bind(bind_address())?
    .run()
    .await
}
