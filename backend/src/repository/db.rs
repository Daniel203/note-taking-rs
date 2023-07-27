use shared::models::note::Note;
use sqlx::{query_as, PgPool};

pub async fn get_notes(pool: &PgPool) -> Result<Vec<Note>, Box<dyn std::error::Error>> {
    let notes: Vec<Note> = query_as!(Note, "SELECT * FROM note")
        .fetch_all(pool)
        .await?;

    return Ok(notes);
}

pub async fn put_note(note: Note, pool: &PgPool) -> Result<(), Box<dyn std::error::Error>> {
    query_as!(
        Note,
        "INSERT INTO note (title, body) VALUES ($1, $2)",
        note.title,
        note.body
    )
    .execute(pool)
    .await?;

    return Ok(());
}
