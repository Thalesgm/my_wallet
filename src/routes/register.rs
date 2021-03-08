use actix_web::{web, HttpResponse};
use chrono::Utc;
use sqlx::PgPool;
use uuid::Uuid;

#[derive(serde::Deserialize)]
pub struct FormData {
    cpf: String,
    email: String,
    name: String,
    senha: String
}

pub async fn register(
    form: web::Form<FormData>,
    pool: web::Data<PgPool>, // Renamed!
) -> Result<HttpResponse, HttpResponse> {
    sqlx::query!(
        r#"
        INSERT INTO users (id, cpf, name, email, password, register_date)
        VALUES ($1, $2, $3, $4, $5, $6)
        "#,
        Uuid::new_v4(),
        form.cpf,
        form.name,
        form.email,
        form.password,
        Utc::now()
    )
    // We got rid of the double-wrapping using .app_data()
    .execute(pool.get_ref())
    .await
    .map_err(|e| {
        eprintln!("Failed to execute query: {}", e);
        HttpResponse::InternalServerError().finish()
    })?;
    Ok(HttpResponse::Ok().finish())
}