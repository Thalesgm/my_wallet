use actix_web::{HttpResponse};

pub async fn manifest() -> HttpResponse {
    HttpResponse::Ok().finish()
}