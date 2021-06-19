use chrono::{DateTime, NaiveDateTime, Utc};
use diesel::sql_types::{Double, Timestamp};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::schema::measurements;

use super::Device;

#[derive(Queryable, Serialize, Associations, Identifiable)]
#[belongs_to(Device, foreign_key = "created_by")]
pub struct Measurement {
    pub id: Uuid,
    pub humidity: f64,
    pub temperature: f64,
    pub carbon_dioxide: f64,
    pub created_at: DateTime<Utc>,
    pub updated_at: Option<DateTime<Utc>>,
    pub location: String,
    pub created_by: Uuid,
}

#[derive(Insertable, Deserialize)]
#[table_name = "measurements"]
pub struct NewMeasurement {
    pub humidity: f64,
    pub temperature: f64,
    pub carbon_dioxide: f64,
    pub location: String,
    pub created_by: Uuid,
}

#[derive(Serialize, Debug, QueryableByName)]
pub struct Stats {
    #[sql_type = "Timestamp"]
    pub moment: NaiveDateTime,
    #[sql_type = "Double"]
    pub humidity: f64,
    #[sql_type = "Double"]
    pub min_humidity: f64,
    #[sql_type = "Double"]
    pub max_humidity: f64,
    #[sql_type = "Double"]
    pub temperature: f64,
    #[sql_type = "Double"]
    pub min_temperature: f64,
    #[sql_type = "Double"]
    pub max_temperature: f64,
}
