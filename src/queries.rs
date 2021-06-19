use diesel::prelude::*;
use diesel::result::Error;
use diesel::{insert_into, sql_query, PgConnection, QueryResult, RunQueryDsl};
use uuid::Uuid;

use crate::models::{Device, Measurement, NewDevice, NewMeasurement, Stats};

use super::schema::devices;
use super::schema::measurements;

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

pub fn find_stats(connection: &PgConnection) -> Result<Vec<Stats>, Error> {
    sql_query(include_str!("sql/stats.sql")).load(connection)
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

pub fn find_devices(connection: &PgConnection) -> Result<Vec<Device>, Error> {
    devices::table
        .order(devices::created_at)
        .load::<Device>(connection)
}
