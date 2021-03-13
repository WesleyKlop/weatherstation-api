extern crate diesel;

use std::process;
use weatherstation_api::database::establish_connection;
use weatherstation_api::models::{save_measurement, NewMeasurement};

fn main() {
    dotenv::dotenv().ok();
    let connection = establish_connection();

    let result = save_measurement(
        NewMeasurement {
            humidity: 70.0,
            temperature: 20.0,
            carbon_dioxide: 600.0,
        },
        &connection,
    );

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
