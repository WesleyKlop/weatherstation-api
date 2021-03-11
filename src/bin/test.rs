extern crate diesel;

use weatherstation_api::establish_connection;
use std::process;
use weatherstation_api::models::{NewMeasurement, create_measurement};

fn main() {
    let connection = establish_connection();

    let result = create_measurement(NewMeasurement {
        humidity: &70.0,
        temperature: &20.0,
        carbon_dioxide: &600.0,
    }, connection);

    match result {
        Ok(true) => {
            println!("Created record!");
            process::exit(0)
        }
        Ok(false) => {
            println!("Failed...");
            process::exit(1)
        }
        Err(msg) => {
            println!("{}", msg);
        }
    }
}