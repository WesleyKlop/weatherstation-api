use crate::database::DbPool;
use crate::models::find_measurements;
use actix_web::{get, web, HttpResponse, Responder};

#[get("/")]
pub async fn all_measurements(pool: web::Data<DbPool>) -> impl Responder {
    let connection = pool.get().expect("Failed to db");

    web::block(move || find_measurements(50, &connection))
        .await
        .map_err(|err| dbg!(err))
        .map(|measurements| HttpResponse::Ok().json(measurements))
}
