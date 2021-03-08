//! src/startup.rs
use crate::routes::{register, deposit, transfer, manifest};
use actix_web::dev::Server;
use actix_web::{web, App, HttpServer};
use sqlx::PgPool;
use std::net::TcpListener;

pub fn run(listener: TcpListener, db_pool: PgPool) -> Result<Server, std::io::Error> {
    // Wrap the pool using web::Data, which boils down to an Arc smart pointer
    let db_pool = web::Data::new(db_pool);
    let server = HttpServer::new(move || {
        App::new()
            .route("/register", web::get().to(register))
            .route("/deposit", web::post().to(deposit))
            .route("/transfer", web::post().to(transfer))
            .route("/manifest", web::get().to(manifest))
            // Our pool is already wrapped in a Data: 
            // using .data would add another Arc pointer on top 
            // of the existing one - an unnecessary indirection.
            // .app_data instead does not perform an additional layer of wrapping.
            .app_data(db_pool.clone())
    })
    .listen(listener)?
    .run();
    Ok(server)
}