use crate::db::{Database, DatabaseConnection};
use async_trait::async_trait;
use mongodb::{options::ClientOptions, Client};
use thiserror::Error;

/// MongoDB-specific error types
#[derive(Error, Debug)]
pub enum MongoError {
    /// Error parsing the MongoDB connection string
    #[error("Failed to parse connection string: {0}")]
    ConnectionStringError(String),
    /// Error creating the MongoDB client
    #[error("Failed to create client: {0}")]
    ClientCreationError(String),
}

/// MongoDB database implementation
pub struct MongoDatabase;

#[async_trait]
impl Database for MongoDatabase {
    type Error = MongoError;
    type Connection = Client;

    /// Establishes a connection to a MongoDB database
    ///
    /// # Arguments
    /// * `url` - MongoDB connection string
    ///
    /// # Returns
    /// - `Ok(Client)` if connection is successful
    /// - `Err(MongoError)` if connection fails
    async fn connect(url: &str) -> Result<Self::Connection, Self::Error> {
        let client_options = ClientOptions::parse(url)
            .await
            .map_err(|e| MongoError::ConnectionStringError(e.to_string()))?;

        Client::with_options(client_options)
            .map_err(|e| MongoError::ClientCreationError(e.to_string()))
    }
}

/// Type alias for MongoDB database connection
pub type MongoConnection = DatabaseConnection<MongoDatabase>;
