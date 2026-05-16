use actix_files::NamedFile;
use actix_web::{get, middleware, web, App, HttpResponse, HttpServer, Responder, HttpRequest};
use rand::Rng;
use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet};
use std::sync::Arc;
use moka::sync::Cache;
use std::time::Duration;

#[derive(Clone, Serialize, Deserialize)]
pub struct Song {
    pub id: usize,
    pub title: String,
    pub artist: String,
    pub album: String,
    pub genre: String,
}

pub struct AppState {
    pub songs_list: Vec<Song>,
    pub song_index: HashMap<String, HashSet<usize>>,
    pub songs_map: HashMap<usize, Song>,
    pub metadata_cache: Cache<usize, Song>,
}

fn generate_mock_data(count: usize) -> (Vec<Song>, HashMap<String, HashSet<usize>>, HashMap<usize, Song>) {
    let mut rng = rand::thread_rng();
    let mut list = Vec::with_capacity(count);
    let mut index: HashMap<String, HashSet<usize>> = HashMap::new();
    let mut map = HashMap::new();

    let genres = ["Rock", "Pop", "Jazz", "Classical", "Hip-Hop", "Electronic"];
    let words = ["Love", "Night", "Day", "Sun", "Moon", "Star", "Heart", "Soul", "Beat", "Song", "Dream", "Fire", "Ice", "Wind", "Storm", "Ocean"];

    for id in 1..=count {
        let title = format!("{} {}", words[rng.gen_range(0..words.len())], words[rng.gen_range(0..words.len())]);
        let artist = format!("Artist {}", rng.gen_range(1..100));
        let album = format!("Album {}", rng.gen_range(1..50));
        let genre = genres[rng.gen_range(0..genres.len())].to_string();

        let song = Song {
            id,
            title: title.clone(),
            artist: artist.clone(),
            album: album.clone(),
            genre: genre.clone(),
        };

        list.push(song.clone());
        map.insert(id, song.clone());

        let text = format!("{} {} {} {}", title, artist, album, genre).to_lowercase();
        for token in text.split_whitespace() {
            index.entry(token.to_string()).or_default().insert(id);
        }
    }

    (list, index, map)
}

#[derive(Deserialize)]
struct SearchQuery {
    q: Option<String>,
}

#[get("/api/search")]
async fn search(data: web::Data<Arc<AppState>>, query: web::Query<SearchQuery>) -> impl Responder {
    let q = query.q.clone().unwrap_or_default().to_lowercase();
    
    let tokens: Vec<&str> = q.split_whitespace().collect();
    if tokens.is_empty() {
        return HttpResponse::Ok().json(Vec::<Song>::new());
    }

    let mut result_ids: Option<HashSet<usize>> = None;
    for token in tokens {
        let mut matching_ids = HashSet::new();
        for (k, v) in &data.song_index {
            if k.contains(token) {
                matching_ids.extend(v);
            }
        }
        
        match result_ids {
            None => {
                result_ids = Some(matching_ids);
            }
            Some(ref mut existing) => {
                existing.retain(|id| matching_ids.contains(id));
            }
        }
    }

    let mut results = Vec::new();
    if let Some(ids) = result_ids {
        for id in ids {
            if let Some(song) = data.songs_map.get(&id) {
                results.push(song.clone());
            }
        }
    }

    HttpResponse::Ok().json(results)
}

#[derive(Deserialize)]
struct PaginationQuery {
    page: Option<usize>,
    limit: Option<usize>,
}

#[get("/api/playlist")]
async fn playlist(data: web::Data<Arc<AppState>>, query: web::Query<PaginationQuery>) -> impl Responder {
    let page = query.page.unwrap_or(1);
    let limit = query.limit.unwrap_or(50);
    
    let start = (page.saturating_sub(1)) * limit;
    let end = std::cmp::min(start + limit, data.songs_list.len());

    if start >= data.songs_list.len() {
        return HttpResponse::Ok().json(Vec::<Song>::new());
    }

    HttpResponse::Ok().json(&data.songs_list[start..end])
}

#[get("/api/metadata/{id}")]
async fn metadata(data: web::Data<Arc<AppState>>, id: web::Path<usize>) -> impl Responder {
    let id_val = id.into_inner();
    if let Some(song) = data.metadata_cache.get(&id_val) {
        return HttpResponse::Ok().json(song);
    }

    if let Some(song) = data.songs_map.get(&id_val) {
        data.metadata_cache.insert(id_val, song.clone());
        return HttpResponse::Ok().json(song);
    }
    
    HttpResponse::NotFound().finish()
}

#[get("/api/stream")]
async fn stream(_req: HttpRequest) -> actix_web::Result<NamedFile> {
    let path = std::path::PathBuf::from("./dummy.mp3");
    Ok(NamedFile::open(path)?.use_last_modified(true))
}

#[get("/")]
async fn home() -> impl Responder {
    HttpResponse::Ok()
        .content_type("text/html")
        .body(include_str!("../templates/index.html"))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "info");
    env_logger::init();

    println!("Generating mock data...");
    let (songs_list, song_index, songs_map) = generate_mock_data(100_000);
    println!("Data generated successfully. Starting server...");

    let metadata_cache = Cache::builder()
        .time_to_live(Duration::from_secs(60))
        .max_capacity(10_000)
        .build();

    let app_state = Arc::new(AppState {
        songs_list,
        song_index,
        songs_map,
        metadata_cache,
    });

    HttpServer::new(move || {
        App::new()
            .wrap(middleware::Logger::default())
            .app_data(web::Data::new(app_state.clone()))
            .service(home)
            .service(search)
            .service(playlist)
            .service(metadata)
            .service(stream)
            .service(actix_files::Files::new("/static", "./static"))
    })
    .bind(("127.0.0.1", 8000))?
    .run()
    .await
}