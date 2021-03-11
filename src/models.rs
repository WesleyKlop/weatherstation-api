use super::schema::measurements;
use super::schema::measurements::dsl::measurements as measurements_table;
use chrono::{NaiveDateTime};
use diesel::{insert_into, Insertable, MysqlConnection, QueryResult, RunQueryDsl};
use diesel::prelude::*;
use diesel::result::Error;
use crate::schema::measurements::columns::created_at;

#[derive(Queryable)]
pub struct Measurement {
    pub uuid: Vec<u8>,
    pub id: String,
    pub humidity: f32,
    pub temperature: f32,
    pub carbon_dioxide: f32,
    pub created_at: NaiveDateTime,
    pub updated_at: Option<NaiveDateTime>,
}

#[derive(Insertable)]
#[table_name = "measurements"]
pub struct NewMeasurement<'a> {
    pub humidity: &'a f32,
    pub temperature: &'a f32,
    pub carbon_dioxide: &'a f32,
}

pub fn create_measurement(measurement: NewMeasurement, connection: MysqlConnection) -> QueryResult<bool> {
    insert_into(measurements_table)
        .values(measurement)
        .execute(&connection)
        .map(|i| i > 0)
}

pub fn find_measurements(limit: i64, connection: MysqlConnection) -> Result<Vec<Measurement>, Error> {
    measurements_table
        .order(created_at)
        .limit(limit)
        .load::<Measurement>(&connection)
}