extern crate diesel;

use std::env;
use uuid::Uuid;
use weatherstation_api::database::establish_connection;
use weatherstation_api::models::find_measurement;

fn main() {
    dotenv::dotenv().ok();

    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        panic!("Missing argument id")
    }
    let connection = establish_connection();
    let id = Uuid::parse_str(&args[1]).expect("id must be a valid uuid");
    let measurement = find_measurement(id, &connection).expect("Error loading measurements");

    println!("Found measurement, created at: {}", measurement.created_at);
    println!(
        "{}°C, {}%, {}ppm",
        measurement.temperature, measurement.humidity, measurement.carbon_dioxide
    );
}
