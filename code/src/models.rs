use mongodb::bson::oid::ObjectId;
use serde::{Deserialize, Serialize};

/// Represents a teacher in the system with their personal information and specialties.
#[derive(Debug, Serialize, Deserialize)]
pub struct Teacher {
    /// MongoDB document ID
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id: Option<ObjectId>,
    /// Initial position in the list
    pub initial_position: i32,
    /// User's first name
    pub name: String,
    /// User's last name
    pub surname: String,
    /// Whether the teacher has services assigned
    pub has_services: bool,
    /// User's specialties and their status
    pub specialties: Specialties,
}

/// Represents the specialties a teacher can have.
/// Each field indicates whether the teacher has that specialty (true) or not (false).
#[derive(Debug, Serialize, Deserialize)]
pub struct Specialties {
    /// Informatics specialty
    pub inf: bool,
    /// Primary education specialty
    pub pri: bool,
    /// English specialty
    pub ing: bool,
    /// French specialty
    pub fra: bool,
    /// Physical Education specialty
    pub ef: bool,
    /// Therapeutic specialty
    pub pt: bool,
    /// Audition and Language specialty
    pub al: bool,
    /// Music specialty
    pub mus: bool,
}
