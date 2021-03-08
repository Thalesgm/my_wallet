use actix_web::{HttpResponse};

pub async fn deposit() -> HttpResponse {
    HttpResponse::Ok().finish()
}