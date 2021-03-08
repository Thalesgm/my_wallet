use actix_web::{HttpResponse};

pub async fn transfer() -> HttpResponse {
    HttpResponse::Ok().finish()
}