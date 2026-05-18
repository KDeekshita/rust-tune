use actix_files::Files;
use actix_web::{get, web, App, HttpResponse, HttpServer, Responder};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone)]
struct Song {
    id: u32,
    title: String,
    artist: String,
    album: String,
}

fn get_songs() -> Vec<Song> {
    vec![
        Song {
            id: 1,
            title: "Blinding Lights".to_string(),
            artist: "The Weeknd".to_string(),
            album: "After Hours".to_string(),
        },
        Song {
            id: 2,
            title: "Believer".to_string(),
            artist: "Imagine Dragons".to_string(),
            album: "Evolve".to_string(),
        },
        Song {
            id: 3,
            title: "Shape of You".to_string(),
            artist: "Ed Sheeran".to_string(),
            album: "Divide".to_string(),
        },
    ]
}

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
    let search_term = query.q.to_lowercase();

    let songs = get_songs();

    let results: Vec<Song> = songs
        .into_iter()
        .filter(|song| {
            song.title.to_lowercase().contains(&search_term)
                || song.artist.to_lowercase().contains(&search_term)
                || song.album.to_lowercase().contains(&search_term)
        })
        .collect();

    HttpResponse::Ok().json(results)
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