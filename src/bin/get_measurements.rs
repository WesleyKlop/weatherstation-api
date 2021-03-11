extern crate diesel;

use weatherstation_api::establish_connection;
use weatherstation_api::models::{Measurement, find_measurements};

fn main() {
    let connection = establish_connection();
    let results: Vec<Measurement> = find_measurements(2000, connection)
        .expect("Error loading measurements");

    println!("Displaying {} measurements", results.len());
    for measurement in results {
        println!("{} | {} °C, {}%, {}ppm", measurement.id, measurement.temperature, measurement.humidity, measurement.carbon_dioxide);
    }
}