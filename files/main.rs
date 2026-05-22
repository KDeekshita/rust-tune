use actix_files::Files;
use actix_web::{get, web, App, HttpServer, Responder};
use serde::Serialize;
use std::env;

// ── Fix 1: Typed response structs via serde ──────────────────────────────────

#[derive(Serialize)]
struct HelloResponse {
    message: String,
    status: String,
}

// ── Fix 2: Song struct + shared app state ────────────────────────────────────

#[derive(Serialize, Clone)]
struct Song {
    id: u32,
    title: String,
    artist: String,
    duration: String,
}

// ── Handlers ─────────────────────────────────────────────────────────────────

#[get("/")]
async fn home() -> impl Responder {
    actix_web::HttpResponse::Ok()
        .content_type("text/html")
        .body(include_str!("../templates/index.html"))
}

#[get("/api/hello")]
async fn hello() -> impl Responder {
    web::Json(HelloResponse {
        message: "Hello from RustTune!".into(),
        status: "ok".into(),
    })
}

#[get("/api/songs")]
async fn songs(data: web::Data<Vec<Song>>) -> impl Responder {
    web::Json(data.get_ref().clone())
}

// ── Main ─────────────────────────────────────────────────────────────────────

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Fix 3: Load .env if present, then read HOST / PORT from environment
    dotenvy::dotenv().ok();

    let host = env::var("HOST").unwrap_or_else(|_| "127.0.0.1".to_string());
    let port: u16 = env::var("PORT")
        .unwrap_or_else(|_| "8000".to_string())
        .parse()
        .expect("PORT must be a valid u16");

    // Seed song data — replace with DB layer when persistence is added
    let song_data: Vec<Song> = vec![
        Song {
            id: 1,
            title: "Midnight Echo".into(),
            artist: "Electronic Vibes".into(),
            duration: "3:24".into(),
        },
        Song {
            id: 2,
            title: "Neon Waves".into(),
            artist: "Synth Pop".into(),
            duration: "4:05".into(),
        },
        Song {
            id: 3,
            title: "Dream Runner".into(),
            artist: "Lo-Fi Beats".into(),
            duration: "2:58".into(),
        },
    ];

    let songs_data = web::Data::new(song_data);

    println!("Server running at http://{host}:{port}");

    HttpServer::new(move || {
        App::new()
            .app_data(songs_data.clone())
            .service(home)
            .service(hello)
            .service(songs)
            .service(Files::new("/static", "./static"))
    })
    .bind((host.as_str(), port))?
    .run()
    .await
}
