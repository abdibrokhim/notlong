use std::env;
use dotenvy::dotenv;

/// Load tokens from the .env file/environment variable.
pub fn get_pg_url() -> String {
    dotenv().ok(); // Initializes .env reading (ignore any load errors).
    env::var("DATABASE_URL").expect("DATABASE_URL environment variable not set")
}