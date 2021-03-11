extern crate diesel;

use std::env;
use weatherstation_api::establish_connection;
use weatherstation_api::models::{find_measurement, Measurement};

fn main() {
    let args: Vec<String> = env::args().collect();
    let connection = establish_connection();
    let measurement: Measurement =
        find_measurement(args[1].to_owned(), connection).expect("Error loading measurements");

    println!("Found measurement, created at: {}", measurement.created_at);
    println!(
        "{}°C, {}%, {}ppm",
        measurement.temperature, measurement.humidity, measurement.carbon_dioxide
    );
}
