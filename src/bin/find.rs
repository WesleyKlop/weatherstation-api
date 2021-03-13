extern crate diesel;

use std::env;
use weatherstation_api::database::establish_connection;
use weatherstation_api::models::find_measurement;

fn main() -> Result<(), &'static str> {
    dotenv::dotenv().ok();

    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        return Err("Missing argument uuid");
    }
    let connection = establish_connection();
    let measurement =
        find_measurement(args[1].to_owned(), &connection).expect("Error loading measurements");

    println!("Found measurement, created at: {}", measurement.created_at);
    println!(
        "{}°C, {}%, {}ppm",
        measurement.temperature, measurement.humidity, measurement.carbon_dioxide
    );

    Ok(())
}
