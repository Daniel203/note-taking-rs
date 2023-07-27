use super::note_tag::NoteTag;
use sqlx::FromRow;
use serde::{Deserialize, Serialize};

#[derive(Debug, FromRow, Serialize, Deserialize)]
pub struct Note {
    pub id: uuid::Uuid,
    pub title: String,
    pub body: Option<String>,
    pub creation_date: chrono::NaiveDate,
    pub last_modification_date: chrono::NaiveDate,
    // pub tag: Option<NoteTag>,
}
