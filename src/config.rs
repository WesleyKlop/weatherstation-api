use std::env;

pub fn bind_address() -> (String, u16) {
    let host = env::var("APP_ADDRESS").expect("APP_ADDRESS must be set");
    let port = env::var("APP_PORT")
        .ok()
        .and_then(|num: String| num.parse::<u16>().ok())
        .expect("APP_PORT must be set and be an integer");

    (host, port)
}

pub fn database_url() -> String {
    env::var("DATABASE_URL").expect("DATABASE_URL must be set")
}
