pub struct NoteTag {
    pub id: uuid::Uuid,
    pub tag_name: String,
    pub color: String,
    pub creation_date: chrono::DateTime<chrono::Local>,
    pub last_modification_date: chrono::DateTime<chrono::Local>,
}
