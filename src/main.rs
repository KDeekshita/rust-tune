mod auth;
mod db;

use actix_files::Files;
use actix_web::{get, web, App, HttpResponse, HttpServer, Responder};
use db::DbPool;
use std::sync::Mutex;

#[get("/")]
async fn home() -> impl Responder {
    HttpResponse::Ok()
        .content_type("text/html")
        .body(include_str!("../templates/index.html"))
}

#[get("/api/hello")]
async fn hello() -> impl Responder {
    HttpResponse::Ok()
        .content_type("application/json")
        .body(r#"{"message":"Hello from RustTune!","status":"ok"}"#)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenvy::dotenv().ok();

    let db_path = std::env::var("DATABASE_URL").unwrap_or_else(|_| "rusttune.db".to_string());
    let conn = db::init_db(&db_path).expect("Failed to initialize database");
    let pool = web::Data::new(DbPool(Mutex::new(conn)));

    println!("Server running at http://127.0.0.1:8000");

    HttpServer::new(move || {
        App::new()
            .app_data(pool.clone())
            // Existing routes — untouched
            .service(home)
            .service(hello)
            // New auth routes
            .route("/api/auth/register", web::post().to(auth::handlers::register))
            .route("/api/auth/login", web::post().to(auth::handlers::login))
            .route("/api/auth/me", web::get().to(auth::handlers::me))
            // Static files — keep last
            .service(Files::new("/static", "./static"))
    })
    .bind(("127.0.0.1", 8000))?
    .run()
    .await
}