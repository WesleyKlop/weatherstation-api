use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::schema::devices;

#[derive(Queryable, Serialize, Identifiable, Clone)]
pub struct Device {
    pub id: Uuid,
    pub location: String,
    pub token: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: Option<DateTime<Utc>>,
}

#[derive(Insertable, Deserialize)]
#[table_name = "devices"]
pub struct NewDevice {
    pub location: String,
    pub token: String,
}
