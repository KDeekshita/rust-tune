use actix_files::Files;
use actix_web::{get, web, App, HttpResponse, HttpServer, Responder};
use serde::Deserialize;

mod models;

#[get("/")]
async fn home() -> impl Responder {
    HttpResponse::Ok()
        .content_type("text/html")
        .body(include_str!("../templates/index.html"))
}

#[derive(Deserialize)]
struct SearchQuery {
    q: String,
}

#[get("/api/search")]
async fn search(query: web::Query<SearchQuery>) -> impl Responder {
    let results = search_songs(&query.q);
    HttpResponse::Ok().json(results)
}

fn search_songs(query: &str) -> Vec<models::Song> {
    let q = query.trim().to_lowercase();
    if q.is_empty() {
        return vec![];
    }
    models::get_all_songs()
        .into_iter()
        .filter(|song| {
            song.title.to_lowercase().contains(&q)
                || song.artist.to_lowercase().contains(&q)
                || song.album.to_lowercase().contains(&q)
        })
        .collect()
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("Server running at http://127.0.0.1:8000");

    HttpServer::new(|| {
        App::new()
            .service(home)
            .service(search)
            .service(Files::new("/static", "./static"))
    })
    .bind(("127.0.0.1", 8000))?
    .run()
    .await
}

#[cfg(test)]
mod tests;