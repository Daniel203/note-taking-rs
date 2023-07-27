use sqlx::FromRow;
use serde::{Deserialize, Serialize};

#[derive(Debug, FromRow, Serialize, Deserialize)]
pub struct NoteTag {
    pub id: uuid::Uuid,
    pub name: String,
    pub color: String,
    pub creation_date: chrono::DateTime<chrono::Local>,
    pub last_modification_date: chrono::DateTime<chrono::Local>,
}
