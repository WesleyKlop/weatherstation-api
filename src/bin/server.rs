use actix_web::dev::ServiceRequest;
use actix_web::error::ErrorUnauthorized;
use actix_web::{middleware, web, web::scope, web::Data, App, Error, HttpMessage, HttpServer};
use actix_web_httpauth::extractors::bearer::BearerAuth;
use actix_web_httpauth::middleware::HttpAuthentication;
use weatherstation_api::config::bind_address;
use weatherstation_api::database::{create_pool, DbPool};
use weatherstation_api::models::find_device_by_token;
use weatherstation_api::routes;

async fn validator(req: ServiceRequest, credentials: BearerAuth) -> Result<ServiceRequest, Error> {
    let pool = req.app_data::<Data<DbPool>>().unwrap();
    let connection = pool.get().expect("Failed to get database connection.");

    web::block(move || find_device_by_token(credentials.token().to_string(), &connection))
        .await
        .map(|device| {
            req.extensions_mut().insert(device);

            req
        })
        .map_err(ErrorUnauthorized)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv::dotenv().ok();
    env_logger::init();

    let pool = create_pool();
    let enable_auth: bool = std::env::var("APP_ENABLE_AUTH") == Ok("true".into());

    HttpServer::new(move || {
        App::new()
            .wrap(middleware::Logger::default())
            .wrap(middleware::NormalizePath::default())
            .data(pool.clone())
            .service(
                scope("/api")
                    .service(
                        scope("/measurements")
                            .wrap(middleware::Condition::new(
                                enable_auth,
                                HttpAuthentication::bearer(validator),
                            ))
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
