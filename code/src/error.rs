use actix_web::{error::ResponseError, HttpResponse};
use derive_more::Display;
use std::error::Error;

/// Application error types
#[derive(Debug, Display)]
#[allow(dead_code)]
pub enum AppError {
    /// Internal server error (500)
    #[display(fmt = "Internal Server Error")]
    InternalError,
    /// Bad request error (400) with a message
    #[display(fmt = "Bad Request: {}", _0)]
    BadRequest(String),
    /// Resource not found error (404)
    #[display(fmt = "Not Found")]
    NotFound,
}

impl Error for AppError {}

impl ResponseError for AppError {
    /// Converts the error into an HTTP response
    ///
    /// # Returns
    /// - `InternalError`: 500 Internal Server Error
    /// - `BadRequest`: 400 Bad Request with message
    /// - `NotFound`: 404 Not Found
    fn error_response(&self) -> HttpResponse {
        match *self {
            AppError::InternalError => {
                HttpResponse::InternalServerError().json("Internal Server Error")
            }
            AppError::BadRequest(ref message) => HttpResponse::BadRequest().json(message),
            AppError::NotFound => HttpResponse::NotFound().json("Not Found"),
        }
    }
}
