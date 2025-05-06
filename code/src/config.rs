use serde::Deserialize;
use std::env;

/// Application configuration loaded from environment variables
#[derive(Debug, Deserialize, Clone)]
pub struct Config {
    /// MongoDB connection URL
    pub database_url: String,
    /// Name of the MongoDB database
    pub database_name: String,
    /// Name of the MongoDB collection
    pub collection_name: String,
    /// Host address to bind the server to
    pub host: String,
    /// Port number to bind the server to
    pub port: u16,
    /// Logging level (debug, info, warn, error)
    pub log_level: String,
}

impl Config {
    /// Creates a new Config instance by loading values from environment variables
    ///
    /// # Environment Variables
    /// - `DATABASE_URL`: MongoDB connection URL (required)
    /// - `DATABASE_NAME`: MongoDB database name (default: "primary")
    /// - `COLLECTION_NAME`: MongoDB collection name (default: "teachers")
    /// - `HOST`: Server host address (default: "127.0.0.1")
    /// - `PORT`: Server port number (default: 8080)
    /// - `LOG_LEVEL`: Logging level (default: "info")
    ///
    /// # Returns
    /// - `Ok(Config)` if all required variables are present
    /// - `Err(env::VarError)` if required variables are missing
    pub fn from_env() -> Result<Self, env::VarError> {
        Ok(Config {
            database_url: env::var("DATABASE_URL")?,
            database_name: env::var("DATABASE_NAME").unwrap_or_else(|_| "primary".to_string()),
            collection_name: env::var("COLLECTION_NAME").unwrap_or_else(|_| "teachers".to_string()),
            host: env::var("HOST").unwrap_or_else(|_| "127.0.0.1".to_string()),
            port: env::var("PORT")
                .unwrap_or_else(|_| "8080".to_string())
                .parse()
                .unwrap_or(8080),
            log_level: env::var("LOG_LEVEL").unwrap_or_else(|_| "info".to_string()),
        })
    }
}
