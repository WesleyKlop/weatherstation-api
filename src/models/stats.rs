use chrono::DateTime;
use chrono::Utc;
use diesel::debug_query;
use diesel::dsl::*;
use diesel::pg::Pg;
use diesel::result::Error;
use diesel::sql_types::{Double, Text, Timestamptz};
use diesel::PgConnection;
use diesel::QueryDsl;
use diesel::RunQueryDsl;

use crate::schema::measurements;

#[derive(Debug, QueryableByName, Queryable)]
pub struct Stats {
    #[diesel(sql_type = Timestamptz, )]
    #[column_name = "moment"]
    pub moment: DateTime<Utc>,
    #[diesel(sql_type = Nullable<Double>)]
    pub humidity: Option<f64>,
    #[diesel(sql_type = Nullable<Double>)]
    pub min_humidity: Option<f64>,
    #[diesel(sql_type = Nullable<Double>)]
    pub max_humidity: Option<f64>,
    #[diesel(sql_type = Nullable<Double>)]
    pub temperature: Option<f64>,
    #[diesel(sql_type = Nullable<Double>)]
    pub min_temperature: Option<f64>,
    #[diesel(sql_type = Nullable<Double>)]
    pub max_temperature: Option<f64>,
}

sql_function!(
    #[aggregate]
    fn date_trunc(text: Text, timestamp: Timestamptz) -> Timestamptz
);

pub fn get_stats(mut conn: PgConnection) -> Result<Vec<Stats>, Error> {
    let trunc_col = sql::<Timestamptz>("date_trunc(")
        .bind::<Text, _>("hour")
        .sql(" , created_at) AS moment");
    let query = measurements::table
        .select((
            trunc_col,
            avg(measurements::humidity),
            min(measurements::humidity),
            max(measurements::humidity),
            avg(measurements::temperature),
            min(measurements::temperature),
            max(measurements::temperature),
        ))
        .group_by(sql::<Text>("moment"));

    let debugq = debug_query::<Pg, _>(&query);
    dbg!(debugq);

    query.load::<Stats>(&mut conn)
}
