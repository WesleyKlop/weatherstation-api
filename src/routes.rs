use super::database::DbPool;
use super::models::{find_measurement, find_measurements, save_measurement, NewMeasurement};
use crate::models::Device;
use actix_web::{get, post, web, HttpResponse, Responder};
use serde::{Deserialize, Serialize};
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

#[derive(Deserialize)]
pub struct NewMeasurementForm {
    pub humidity: f64,
    pub temperature: f64,
    pub carbon_dioxide: f64,
}

#[post("/")]
pub async fn create_measurement(
    request: web::HttpRequest,
    pool: web::Data<DbPool>,
    new_measurement: web::Json<NewMeasurementForm>,
) -> impl Responder {
    let connection = pool.get().expect("Failed to db");
    let device: Device = request.extensions().get().cloned().unwrap();

    web::block(move || {
        save_measurement(
            NewMeasurement {
                humidity: new_measurement.humidity,
                temperature: new_measurement.temperature,
                carbon_dioxide: new_measurement.carbon_dioxide,
                created_by: device.id,
            },
            &connection,
        )
    })
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
