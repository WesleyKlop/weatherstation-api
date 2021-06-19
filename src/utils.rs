use crate::database::DbPool;
use crate::queries::find_device_by_token;
use actix_web::dev::ServiceRequest;
use actix_web::error::ErrorUnauthorized;
use actix_web::{web, Error, HttpMessage};
use actix_web_httpauth::extractors::bearer::BearerAuth;

pub async fn validator(
    req: ServiceRequest,
    credentials: BearerAuth,
) -> Result<ServiceRequest, Error> {
    let pool = req.app_data::<web::Data<DbPool>>().unwrap();
    let connection = pool.get().expect("Failed to get database connection.");

    web::block(move || find_device_by_token(credentials.token().to_string(), &connection))
        .await
        .map(|device| {
            req.extensions_mut().insert(device);

            req
        })
        .map_err(ErrorUnauthorized)
}
