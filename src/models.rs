use super::schema::devices;
use super::schema::measurements;
use chrono::NaiveDateTime;
use diesel::prelude::*;
use diesel::result::Error;
use diesel::{insert_into, Insertable, PgConnection, QueryResult, RunQueryDsl};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Queryable, Serialize, Associations, Identifiable)]
#[belongs_to(Device, foreign_key = "created_by")]
pub struct Measurement {
    pub id: Uuid,
    pub humidity: f64,
    pub temperature: f64,
    pub carbon_dioxide: f64,
    pub created_at: NaiveDateTime,
    pub updated_at: Option<NaiveDateTime>,
    pub location: String,
    pub created_by: Uuid,
}

#[derive(Insertable, Deserialize)]
#[table_name = "measurements"]
pub struct NewMeasurement {
    pub humidity: f64,
    pub temperature: f64,
    pub carbon_dioxide: f64,
}

pub fn save_measurement(
    measurement: NewMeasurement,
    connection: &PgConnection,
) -> QueryResult<Measurement> {
    insert_into(measurements::table)
        .values(measurement)
        .get_result(connection)
}

pub fn find_measurements(limit: i64, connection: &PgConnection) -> Result<Vec<Measurement>, Error> {
    assert!(limit > 0);
    measurements::table
        .order(measurements::created_at)
        .limit(limit)
        .load::<Measurement>(connection)
}

pub fn find_measurement(id: Uuid, connection: &PgConnection) -> Result<Measurement, Error> {
    measurements::table
        .filter(measurements::id.eq(id))
        .first(connection)
}

#[derive(Queryable, Serialize, Identifiable)]
pub struct Device {
    pub id: Uuid,
    pub location: String,
    pub token: String,
    pub created_at: NaiveDateTime,
    pub updated_at: Option<NaiveDateTime>,
}

#[derive(Insertable, Deserialize)]
#[table_name = "devices"]
pub struct NewDevice {
    pub location: String,
    pub token: String,
}

pub fn find_device_by_token(token: String, connection: &PgConnection) -> Result<Device, Error> {
    devices::table
        .filter(devices::token.eq(token))
        .first(connection)
}

pub fn register_device(device: NewDevice, connection: &PgConnection) -> QueryResult<Device> {
    insert_into(devices::table)
        .values(device)
        .get_result(connection)
}
