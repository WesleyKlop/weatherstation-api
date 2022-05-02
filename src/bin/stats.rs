extern crate dotenv;

use color_eyre::eyre::Result;
use dotenv::dotenv;
use weatherstation_api::establish_connection;
use weatherstation_api::models::stats;

fn main() -> Result<()> {
    color_eyre::install()?;
    dotenv().ok();

    let conn = establish_connection();

    let stats = stats::get_stats(conn)?;

    for row in stats {
        dbg!(row);
    }
    Ok(())
}
