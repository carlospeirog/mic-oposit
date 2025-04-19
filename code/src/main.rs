use actix_web::{web, App, HttpServer};
use env_logger::Env;
use log::info;

mod api;
mod config;
mod db;
mod error;
mod models;
use api::users::MongoUserRepository;
use db::mongodb::MongoConnection;

/// Main entry point for the mic-oposit application
///
/// This function:
/// 1. Loads environment variables from .env file
/// 2. Initializes the application configuration
/// 3. Sets up logging
/// 4. Establishes a connection to MongoDB
/// 5. Starts the HTTP server
///
/// # Environment Variables
/// See `Config::from_env()` for required environment variables
///
/// # Returns
/// - `Ok(())` if the server runs successfully
/// - `Err(std::io::Error)` if the server fails to start
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Load environment variables from .env file
    dotenv::dotenv().ok();

    // Load configuration
    let config = config::Config::from_env().expect("Failed to load configuration");
    info!(
        "Starting mic-oposit server on {}:{}",
        config.host, config.port
    );

    // Initialize logger
    env_logger::Builder::from_env(Env::default().default_filter_or(config.log_level.as_str()))
        .init();

    // Initialize database
    let db_connection = MongoConnection::new(&config.database_url)
        .await
        .expect("Failed to initialize database");
    let db = db_connection.connection.database(&config.database_name);
    let collection = db.collection(&config.collection_name);
    let user_repo = web::Data::new(MongoUserRepository::new(collection));
    let config_data = web::Data::new(config.clone());

    // Start HTTP server
    HttpServer::new(move || {
        App::new()
            .app_data(user_repo.clone())
            .app_data(config_data.clone())
            .configure(api::config)
    })
    .bind((config.host, config.port))?
    .run()
    .await
}
