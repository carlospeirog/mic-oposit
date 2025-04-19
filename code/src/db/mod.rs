pub mod mongodb;

use async_trait::async_trait;
use std::error::Error;

/// Trait defining the interface for database connections
#[async_trait]
pub trait Database: Send + Sync {
    /// The error type returned by database operations
    type Error: Error + Send + Sync + 'static;
    /// The connection type used to interact with the database
    type Connection;

    /// Establishes a connection to the database
    ///
    /// # Arguments
    /// * `url` - Connection string for the database
    ///
    /// # Returns
    /// - `Ok(Self::Connection)` if connection is successful
    /// - `Err(Self::Error)` if connection fails
    async fn connect(url: &str) -> Result<Self::Connection, Self::Error>;
}

/// Wrapper around a database connection
#[allow(dead_code)]
pub struct DatabaseConnection<DB: Database> {
    /// The actual database connection
    pub connection: DB::Connection,
}

impl<DB: Database> DatabaseConnection<DB> {
    /// Creates a new database connection
    ///
    /// # Arguments
    /// * `url` - Connection string for the database
    ///
    /// # Returns
    /// - `Ok(Self)` if connection is successful
    /// - `Err(DB::Error)` if connection fails
    pub async fn new(url: &str) -> Result<Self, DB::Error> {
        let connection = DB::connect(url).await?;
        Ok(Self { connection })
    }
}
