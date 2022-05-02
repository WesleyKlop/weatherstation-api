use rand::{distributions::Alphanumeric, Rng};
use std::env;
use weatherstation_api::database::establish_connection;
use weatherstation_api::models::NewDevice;
use weatherstation_api::queries::register_device;

fn main() {
    dotenv::dotenv().ok();
    env_logger::init();

    let args: Vec<String> = env::args().collect();
    let pool = establish_connection();

    let token = match args.len() {
        2 => rand::thread_rng()
            .sample_iter(&Alphanumeric)
            .take(32)
            .map(char::from)
            .collect(),
        3 => args[2].to_string(),
        _ => {
            panic!("What are you trying to do?");
        }
    };

    if token.len() != 32 {
        panic!("Token must have a length of 32, but got {}", token.len());
    }

    let device = register_device(
        NewDevice {
            location: args[1].to_string(),
            token,
        },
        &pool,
    )
    .expect("Failed to create device");

    println!(
        "Created device for location {} with token {}",
        device.location, device.token
    );
}
