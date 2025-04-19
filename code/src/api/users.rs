use crate::error::AppError;
use crate::models::User;
use actix_web::{get, web, HttpResponse, Responder};
use futures::TryStreamExt;
use mongodb::{bson::doc, bson::oid::ObjectId, Collection};
use serde::Deserialize;
use std::error::Error;

/// Trait defining the interface for user data access
#[async_trait::async_trait]
pub trait UserRepository: Send + Sync {
    type Error: Error + Send + Sync + 'static;

    /// Find users matching the given criteria
    async fn find_users(
        &self,
        name: Option<&str>,
        surname: Option<&str>,
        specialty: Option<&str>,
    ) -> Result<Vec<User>, Self::Error>;

    /// Find a user by their ID
    async fn find_user_by_id(&self, id: &str) -> Result<Option<User>, Self::Error>;
}

/// MongoDB implementation of UserRepository
pub struct MongoUserRepository {
    collection: Collection<User>,
}

impl MongoUserRepository {
    pub fn new(collection: Collection<User>) -> Self {
        Self { collection }
    }
}

#[async_trait::async_trait]
impl UserRepository for MongoUserRepository {
    type Error = mongodb::error::Error;

    async fn find_users(
        &self,
        name: Option<&str>,
        surname: Option<&str>,
        specialty: Option<&str>,
    ) -> Result<Vec<User>, Self::Error> {
        let mut filter = doc! {};
        if let Some(name) = name {
            filter.insert("name", name.to_uppercase());
        }
        if let Some(surname) = surname {
            filter.insert(
                "surname",
                doc! {
                    "$regex": surname.to_uppercase(),
                    "$options": ""
                },
            );
        }
        if let Some(specialty) = specialty {
            filter.insert(format!("specialties.{}", specialty), true);
        }

        let cursor = self.collection.find(filter).await?;
        cursor.try_collect().await
    }

    async fn find_user_by_id(&self, id: &str) -> Result<Option<User>, Self::Error> {
        let object_id = match ObjectId::parse_str(id) {
            Ok(id) => id,
            Err(_) => return Ok(None),
        };

        let filter = doc! { "_id": object_id };
        self.collection.find_one(filter).await
    }
}

/// Query parameters for filtering users
#[derive(Debug, Deserialize)]
pub struct UserQuery {
    /// Filter by first name (case-insensitive)
    name: Option<String>,
    /// Filter by last name (case-insensitive, partial match)
    surname: Option<String>,
    /// Filter by specialty name
    specialty: Option<String>,
}

/// Get a list of users with optional filtering
///
/// # Query Parameters
/// - `name`: Filter by first name (case-insensitive)
/// - `surname`: Filter by last name (case-insensitive, partial match)
/// - `specialty`: Filter by specialty name
///
/// # Returns
/// - 200 OK with array of users
/// - 500 Internal Server Error if database operation fails
#[get("/users")]
async fn get_users(
    repo: web::Data<MongoUserRepository>,
    query: web::Query<UserQuery>,
) -> Result<impl Responder, AppError> {
    let users = repo
        .find_users(
            query.name.as_deref(),
            query.surname.as_deref(),
            query.specialty.as_deref(),
        )
        .await
        .map_err(|_| AppError::InternalError)?;

    if users.is_empty() {
        Err(AppError::NotFound)
    } else {
        Ok(HttpResponse::Ok().json(users))
    }
}

/// Get a single user by their ID
///
/// # Path Parameters
/// - `id`: MongoDB ObjectId of the user
///
/// # Returns
/// - 200 OK with user data
/// - 404 Not Found if user doesn't exist
/// - 500 Internal Server Error if database operation fails
#[get("/users/{id}")]
async fn get_user_by_id(
    repo: web::Data<MongoUserRepository>,
    id: web::Path<String>,
) -> Result<impl Responder, AppError> {
    match repo
        .find_user_by_id(&id)
        .await
        .map_err(|_| AppError::InternalError)?
    {
        Some(user) => Ok(HttpResponse::Ok().json(user)),
        None => Err(AppError::NotFound),
    }
}

/// Configure the users API routes
pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(get_users).service(get_user_by_id);
}
