use chrono::DateTime;
use chrono::Utc;
use uuid::Uuid;

use crate::schema::measurements;

#[derive(Queryable)]
#[diesel(belongs_to(Device))]
pub struct Measurement {
    pub id: Uuid,
    pub humidity: f64,
    pub temperature: f64,
    pub carbon_dioxide: f64,
    pub location: String,
    pub created_by: Uuid,
    pub created_at: DateTime<Utc>,
    pub updated_at: Option<DateTime<Utc>>,
}

#[derive(Insertable)]
#[diesel(table_name = measurements)]
pub struct NewMeasurement<'a> {
    pub location: &'a str,
    pub humidity: &'a f64,
    pub temperature: &'a f64,
    pub carbon_dioxide: &'a f64,
    pub created_by: &'a Uuid,
}
