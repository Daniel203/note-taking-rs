use actix_web::{get, post, web::{self, Json}, HttpResponse, Responder};
use shared::models::note::Note;

use crate::{
    app_state::AppState,
    repository::db,
};

pub fn service_config(cfg: &mut web::ServiceConfig) {
    cfg.service(web::scope("/notes").service(get_notes).service(put_note));
}

#[get("")]
async fn get_notes(data: web::Data<AppState>) -> impl Responder {
    match db::get_notes(&data.pool).await {
        Ok(res) => return HttpResponse::Ok().json(res),
        Err(err) => return HttpResponse::InternalServerError().body(err.to_string()),
    };
}

#[post("")]
async fn put_note(request: Json<Note>, data: web::Data<AppState>) -> impl Responder {
    match db::put_note(request.into_inner(), &data.pool).await {
        Ok(_) => return HttpResponse::Ok().body("Note added"),
        Err(err) => return HttpResponse::InternalServerError().body(err.to_string()),
    };
}
