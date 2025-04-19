use crate::error::AppError;
use crate::models::Teacher;
use actix_web::{get, web, HttpResponse, Responder};
use futures::TryStreamExt;
use mongodb::{bson::doc, bson::oid::ObjectId, Collection};
use serde::Deserialize;
use std::error::Error;

/// Trait defining the interface for teacher data access
#[async_trait::async_trait]
pub trait UserRepository: Send + Sync {
    type Error: Error + Send + Sync + 'static;

    /// Find teachers matching the given criteria
    async fn find_teachers(
        &self,
        name: Option<&str>,
        surname: Option<&str>,
        specialty: Option<&str>,
    ) -> Result<Vec<Teacher>, Self::Error>;

    /// Find a teacher by their ID
    async fn find_teacher_by_id(&self, id: &str) -> Result<Option<Teacher>, Self::Error>;
}

/// MongoDB implementation of UserRepository
pub struct MongoUserRepository {
    collection: Collection<Teacher>,
}

impl MongoUserRepository {
    pub fn new(collection: Collection<Teacher>) -> Self {
        Self { collection }
    }
}

#[async_trait::async_trait]
impl UserRepository for MongoUserRepository {
    type Error = mongodb::error::Error;

    async fn find_teachers(
        &self,
        name: Option<&str>,
        surname: Option<&str>,
        specialty: Option<&str>,
    ) -> Result<Vec<Teacher>, Self::Error> {
        let mut filter = doc! {};
        if let Some(name) = name {
            filter.insert(
                "name",
                doc! {
                    "$regex": name.to_uppercase(),
                    "$options": ""
                },
            );
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

    async fn find_teacher_by_id(&self, id: &str) -> Result<Option<Teacher>, Self::Error> {
        let object_id = match ObjectId::parse_str(id) {
            Ok(id) => id,
            Err(_) => return Ok(None),
        };

        let filter = doc! { "_id": object_id };
        self.collection.find_one(filter).await
    }
}

/// Query parameters for filtering teachers
#[derive(Debug, Deserialize)]
pub struct TeacherQuery {
    /// Filter by first name (case-insensitive)
    name: Option<String>,
    /// Filter by last name (case-insensitive, partial match)
    surname: Option<String>,
    /// Filter by specialty name
    specialty: Option<String>,
}

/// Get a list of teachers with optional filtering
///
/// # Query Parameters
/// - `name`: Filter by first name (case-insensitive)
/// - `surname`: Filter by last name (case-insensitive, partial match)
/// - `specialty`: Filter by specialty name
///
/// # Returns
/// - 200 OK with array of teachers
/// - 500 Internal Server Error if database operation fails
#[get("/teachers")]
async fn get_teachers(
    repo: web::Data<MongoUserRepository>,
    query: web::Query<TeacherQuery>,
) -> Result<impl Responder, AppError> {
    let teachers = repo
        .find_teachers(
            query.name.as_deref(),
            query.surname.as_deref(),
            query.specialty.as_deref(),
        )
        .await
        .map_err(|_| AppError::InternalError)?;

    if teachers.is_empty() {
        Err(AppError::NotFound)
    } else {
        Ok(HttpResponse::Ok().json(teachers))
    }
}

/// Get a single teacher by their ID
///
/// # Path Parameters
/// - `id`: MongoDB ObjectId of the teacher
///
/// # Returns
/// - 200 OK with teacher data
/// - 404 Not Found if teacher doesn't exist
/// - 500 Internal Server Error if database operation fails
#[get("/teachers/{id}")]
async fn get_teacher_by_id(
    repo: web::Data<MongoUserRepository>,
    id: web::Path<String>,
) -> Result<impl Responder, AppError> {
    match repo
        .find_teacher_by_id(&id)
        .await
        .map_err(|_| AppError::InternalError)?
    {
        Some(teacher) => Ok(HttpResponse::Ok().json(teacher)),
        None => Err(AppError::NotFound),
    }
}

/// Configure the teachers API routes
pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(get_teachers).service(get_teacher_by_id);
}
