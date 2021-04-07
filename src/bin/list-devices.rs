use weatherstation_api::database::establish_connection;
use weatherstation_api::models::find_devices;

fn main() {
    dotenv::dotenv().ok();
    env_logger::init();

    let pool = establish_connection();

    let devices = find_devices(&pool).expect("Failed to get devices");

    println!("{:12}  |  {:32}  |  {:30}", "location", "token", "added at");
    for device in devices {
        println!(
            "{:12}  |  {:32}  |  {:30}",
            device.location, device.token, device.created_at,
        );
    }
}
