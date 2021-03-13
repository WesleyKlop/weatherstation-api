extern crate diesel;

use weatherstation_api::database::establish_connection;
use weatherstation_api::models::{save_measurement, NewMeasurement};

fn main() {
    dotenv::dotenv().ok();
    let connection = establish_connection();

    let measurement = save_measurement(
        NewMeasurement {
            humidity: 70.0,
            temperature: 20.0,
            carbon_dioxide: 600.0,
        },
        &connection,
    )
    .expect("Failed to create measurement");

    println!("Found measurement, created at: {}", measurement.created_at);
    println!(
        "{}°C, {}%, {}ppm",
        measurement.temperature, measurement.humidity, measurement.carbon_dioxide
    );
}
