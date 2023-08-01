use super::note_tag::NoteTag;
use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Debug, FromRow, Serialize, Deserialize)]
pub struct Note {
    pub id: Option<uuid::Uuid>,
    pub title: String,
    pub body: Option<String>,
    pub creation_date: Option<chrono::NaiveDate>,
    pub last_modification_date: Option<chrono::NaiveDate>,
    // pub tag: Option<NoteTag>,
}

impl Default for Note {
    fn default() -> Self {
        return Note {
            id: None,
            title: "".to_string(),
            body: None,
            creation_date: None,
            last_modification_date: None,
        };

    }
}
