use dotenvy::dotenv;
use std::env;

/// Configuration struct to hold environment variables.
#[derive(Debug, Clone)]
pub struct Config {
    pub server_port: u16,
    pub webhook_path: String,
    // Add other secrets here (e.g., Discord Webhook URL)
}

impl Config {
    /// Loads configuration from the .env file and environment variables.
    /// Panics if required variables are missing.
    pub fn from_env() -> Self {
        dotenv().ok(); // Load .env file if it exists

        let server_port = env::var("SERVER_PORT")
            .unwrap_or_else(|_| "3000".to_string())
            .parse::<u16>()
            .expect("SERVER_PORT must be a valid number");

        let webhook_path =
            env::var("WEBHOOK_PATH").unwrap_or_else(|_| "/webhook/saweria".to_string());

        Config {
            server_port,
            webhook_path,
        }
    }
}
