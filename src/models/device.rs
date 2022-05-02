use chrono::DateTime;
use chrono::Utc;
use uuid::Uuid;

use crate::schema::devices;

#[derive(Queryable)]
pub struct Device {
    pub id: Uuid,
    pub location: String,
    pub token: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: Option<DateTime<Utc>>,
}

#[derive(Insertable)]
#[diesel(table_name = devices)]
pub struct NewDevice<'a> {
    pub location: &'a str,
    pub token: &'a str,
}
