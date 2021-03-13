use super::schema::measurements;
use super::schema::measurements::dsl::measurements as measurements_table;
use crate::schema::measurements::columns::{created_at, id};
use chrono::NaiveDateTime;
use diesel::prelude::*;
use diesel::result::Error;
use diesel::{insert_into, Insertable, PgConnection, QueryResult, RunQueryDsl};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Queryable, Serialize)]
pub struct Measurement {
    pub id: Uuid,
    pub humidity: f64,
    pub temperature: f64,
    pub carbon_dioxide: f64,
    pub created_at: NaiveDateTime,
    pub updated_at: Option<NaiveDateTime>,
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
    insert_into(measurements_table)
        .values(measurement)
        .get_result(connection)
}

pub fn find_measurements(limit: i64, connection: &PgConnection) -> Result<Vec<Measurement>, Error> {
    assert!(limit > 0);
    measurements_table
        .order(created_at)
        .limit(limit)
        .load::<Measurement>(connection)
}

pub fn find_measurement(the_uuid: Uuid, connection: &PgConnection) -> Result<Measurement, Error> {
    measurements_table.filter(id.eq(the_uuid)).first(connection)
}
