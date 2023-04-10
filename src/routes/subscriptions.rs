use actix_web::{web, HttpResponse};
use serde::Deserialize;

#[derive(Deserialize)]
pub struct FormData {
    email: String,
    name: String,
}

pub async fn subscribe(form: web::Form<FormData>) -> HttpResponse {
    // Form is a extractor offered by actix_web to extract inform from request
    HttpResponse::Ok().finish()
}
