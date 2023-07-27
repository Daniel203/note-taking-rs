use actix_web::{get, HttpResponse};

pub mod notes;

#[get("")]
pub async fn default_route() -> HttpResponse {
    return HttpResponse::Ok().body("Api for the note-taking-rs app");
}
