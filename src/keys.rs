use std::env;
use dotenvy::dotenv;

pub fn get_pg_url() -> String {
    dotenv().ok(); // Initializes .env reading (ignore any load errors).
    env::var("DATABASE_URL").expect("DATABASE_URL environment variable not set")
}