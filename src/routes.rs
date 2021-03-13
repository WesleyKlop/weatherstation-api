use crate::database::DbPool;
use crate::models::{find_measurement, find_measurements, save_measurement, NewMeasurement};
use actix_web::{get, post, web, HttpResponse, Responder};
use serde::Serialize;
use uuid::Uuid;

#[get("/")]
pub async fn all_measurements(pool: web::Data<DbPool>) -> impl Responder {
    let connection = pool.get().expect("Failed to db");

    web::block(move || find_measurements(50, &connection))
        .await
        .map(|measurements| HttpResponse::Ok().json(measurements))
}

#[get("/{id}/")]
pub async fn measurement_by_id(pool: web::Data<DbPool>, id: web::Path<Uuid>) -> impl Responder {
    let connection = pool.get().expect("Failed to db");

    web::block(move || find_measurement(id.into_inner(), &connection))
        .await
        .map(|measurement| HttpResponse::Ok().json(measurement))
}

#[post("/")]
pub async fn create_measurement(
    pool: web::Data<DbPool>,
    new_measurement: web::Json<NewMeasurement>,
) -> impl Responder {
    let connection = pool.get().expect("Failed to db");

    web::block(move || save_measurement(new_measurement.into_inner(), &connection))
        .await
        .map(|measurements| HttpResponse::Created().json(measurements))
}

#[get("/health/")]
pub async fn health() -> impl Responder {
    #[derive(Serialize)]
    struct Health {
        status: String,
    }

    HttpResponse::Ok().json(Health {
        status: "ok".to_string(),
    })
}
