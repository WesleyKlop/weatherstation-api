use super::schema::measurements;
use super::schema::measurements::dsl::measurements as measurements_table;
use crate::schema::measurements::columns::{created_at, id};
use chrono::NaiveDateTime;
use diesel::prelude::*;
use diesel::result::Error;
use diesel::{insert_into, Insertable, MysqlConnection, QueryResult, RunQueryDsl};
use serde::{Deserialize, Serialize};

#[derive(Queryable, Serialize)]
pub struct Measurement {
    #[serde(skip_serializing)]
    pub uuid: Vec<u8>,
    pub id: String,
    pub humidity: f32,
    pub temperature: f32,
    pub carbon_dioxide: f32,
    pub created_at: NaiveDateTime,
    pub updated_at: Option<NaiveDateTime>,
}

#[derive(Insertable, Deserialize)]
#[table_name = "measurements"]
pub struct NewMeasurement {
    pub humidity: f32,
    pub temperature: f32,
    pub carbon_dioxide: f32,
}

pub fn save_measurement(
    measurement: NewMeasurement,
    connection: &MysqlConnection,
) -> QueryResult<bool> {
    insert_into(measurements_table)
        .values(measurement)
        .execute(connection)
        .map(|i| i > 0)
}

pub fn find_measurements(
    limit: i64,
    connection: &MysqlConnection,
) -> Result<Vec<Measurement>, Error> {
    assert!(limit > 0);
    measurements_table
        .order(created_at)
        .limit(limit)
        .load::<Measurement>(connection)
}

pub fn find_measurement(
    the_uuid: String,
    connection: &MysqlConnection,
) -> Result<Measurement, Error> {
    measurements_table.filter(id.eq(the_uuid)).first(connection)
}
