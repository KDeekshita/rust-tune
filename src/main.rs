use std::sync::Mutex;

use actix_files::Files;
use actix_web::{App, HttpResponse, HttpServer, Responder, get, web};

use crate::{db::inmemory::Db, routes::auth::{signin, signup}};
pub mod routes;
pub mod types;
pub mod db;

#[get("/")]
async fn home() -> impl Responder {
    HttpResponse::Ok()
        .content_type("text/html")
        .body(include_str!("../templates/index.html"))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {

    let db = web::Data::new(Mutex::new(Db::new()));

    HttpServer::new(move || {
        App::new()
        .app_data(db.clone())
            .service(home)
            .service(
                web::scope("/auth")
                .route("/signin", web::post().to(signin))
                .route("/signup", web::post().to(signup))
            )
            .service(Files::new("/static", "./static"))
           
    })
    .bind(("127.0.0.1", 8000))?
    .run()
    .await
}