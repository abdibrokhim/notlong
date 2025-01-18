use std::env;
use dotenvy::dotenv;
use std::sync::OnceLock;

// Use OnceLock for lazy static initialization of the key
static CRYPTO_KEY: OnceLock<[u8; 32]> = OnceLock::new();

pub fn get_pg_url() -> String {
    dotenv().ok(); // Initializes .env reading (ignore any load errors).
    env::var("DATABASE_URL").expect("DATABASE_URL environment variable not set")
}

/// Gets the crypto key from environment, initializing it only once
pub fn get_crypto_key() -> &'static [u8; 32] {
    CRYPTO_KEY.get_or_init(|| {
        dotenv().ok(); // Load .env file if it exists
        let key_str = env::var("CRYPTO_KEY")
            .expect("CRYPTO_KEY environment variable not set");
        
        let key_bytes = hex::decode(&key_str)
            .expect("CRYPTO_KEY must be a 64-character hex string");
        
        // Ensure the key is exactly 32 bytes
        if key_bytes.len() != 32 {
            panic!("CRYPTO_KEY must be exactly 32 bytes (64 hex characters) long");
        }
        
        let mut key = [0u8; 32];
        key.copy_from_slice(&key_bytes);
        key
    })
}