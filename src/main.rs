use actix_files::Files;
use actix_web::{get, put, web, App, HttpResponse, HttpServer, Responder};
use serde::{Deserialize, Serialize};
use std::sync::Mutex;

#[derive(Serialize, Clone)]
struct Song {
    id: u32,
    title: String,
    artist: String,
    duration: String,
    url: String,
}

#[derive(Deserialize, Serialize)]
struct UpdateSong {
    title: String,
    artist: String,
    duration: String,
    url: String,
}

#[derive(Serialize)]
struct ApiResponse<T: Serialize> {
    success: bool,
    data: Option<T>,
    message: String,
}

// NOTE: Static/mock song catalog. Replace with a real data source
// (database, external API, etc.) when persistence is introduced.
pub(crate) fn songs() -> Vec<Song> {
    vec![
        Song {
            id: 1,
            title: "SoundHelix Song 1".to_string(),
            artist: "SoundHelix".to_string(),
            duration: "6:13".to_string(),
            url: "https://www.soundhelix.com/examples/mp3/SoundHelix-Song-1.mp3".to_string(),
        },
        Song {
            id: 2,
            title: "SoundHelix Song 2".to_string(),
            artist: "SoundHelix".to_string(),
            duration: "6:08".to_string(),
            url: "https://www.soundhelix.com/examples/mp3/SoundHelix-Song-2.mp3".to_string(),
        },
        Song {
            id: 3,
            title: "SoundHelix Song 3".to_string(),
            artist: "SoundHelix".to_string(),
            duration: "6:18".to_string(),
            url: "https://www.soundhelix.com/examples/mp3/SoundHelix-Song-3.mp3".to_string(),
        },
        Song {
            id: 4,
            title: "SoundHelix Song 4".to_string(),
            artist: "SoundHelix".to_string(),
            duration: "4:54".to_string(),
            url: "https://www.soundhelix.com/examples/mp3/SoundHelix-Song-4.mp3".to_string(),
        },
        Song {
            id: 5,
            title: "SoundHelix Song 5".to_string(),
            artist: "SoundHelix".to_string(),
            duration: "6:42".to_string(),
            url: "https://www.soundhelix.com/examples/mp3/SoundHelix-Song-5.mp3".to_string(),
        },
        Song {
            id: 6,
            title: "SoundHelix Song 6".to_string(),
            artist: "SoundHelix".to_string(),
            duration: "6:25".to_string(),
            url: "https://www.soundhelix.com/examples/mp3/SoundHelix-Song-6.mp3".to_string(),
        },
        Song {
            id: 7,
            title: "SoundHelix Song 7".to_string(),
            artist: "SoundHelix".to_string(),
            duration: "5:59".to_string(),
            url: "https://www.soundhelix.com/examples/mp3/SoundHelix-Song-7.mp3".to_string(),
        },
        Song {
            id: 8,
            title: "SoundHelix Song 8".to_string(),
            artist: "SoundHelix".to_string(),
            duration: "5:13".to_string(),
            url: "https://www.soundhelix.com/examples/mp3/SoundHelix-Song-8.mp3".to_string(),
        },
        Song {
            id: 9,
            title: "SoundHelix Song 9".to_string(),
            artist: "SoundHelix".to_string(),
            duration: "4:30".to_string(),
            url: "https://www.soundhelix.com/examples/mp3/SoundHelix-Song-9.mp3".to_string(),
        },
        Song {
            id: 10,
            title: "SoundHelix Song 10".to_string(),
            artist: "SoundHelix".to_string(),
            duration: "6:01".to_string(),
            url: "https://www.soundhelix.com/examples/mp3/SoundHelix-Song-10.mp3".to_string(),
        },
    ]
}

struct AppState {
    songs: Mutex<Vec<Song>>,
}

#[get("/")]
async fn home() -> impl Responder {
    HttpResponse::Ok()
        .content_type("text/html")
        .body(include_str!("../templates/index.html"))
}

#[get("/api/songs")]
async fn list_songs(data: web::Data<AppState>) -> impl Responder {
    let songs = data.songs.lock().unwrap();
    HttpResponse::Ok().json(&*songs)
}

#[put("/api/songs/{id}")]
async fn update_song(
    path: web::Path<u32>,
    body: web::Json<UpdateSong>,
    data: web::Data<AppState>,
) -> impl Responder {
    let id = path.into_inner();
    let mut songs = data.songs.lock().unwrap();

    match songs.iter_mut().find(|s| s.id == id) {
        Some(song) => {
            song.title = body.title.clone();
            song.artist = body.artist.clone();
            song.duration = body.duration.clone();
            song.url = body.url.clone();
            HttpResponse::Ok().json(ApiResponse {
                success: true,
                data: Some(song.clone()),
                message: "Song updated successfully".to_string(),
            })
        }
        None => HttpResponse::NotFound().json(ApiResponse::<Song> {
            success: false,
            data: None,
            message: format!("Song with id {} not found", id),
        }),
    }
}

#[get("/api/hello")]
async fn hello() -> impl Responder {
    HttpResponse::Ok()
        .content_type("application/json")
        .body(r#"{"message":"Hello from RustTune!","status":"ok"}"#)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("Server running at http://127.0.0.1:8000");

    let state = web::Data::new(AppState {
        songs: Mutex::new(songs()),
    });

    HttpServer::new(move || {
        App::new()
            .app_data(state.clone())
            .service(home)
            .service(list_songs)
            .service(update_song)
            .service(hello)
            .service(Files::new("/static", "./static"))
    })
    .bind(("127.0.0.1", 8000))?
    .run()
    .await
}

#[cfg(test)]
mod tests {
    use super::*;
    use actix_web::{test, App};

    fn test_state() -> web::Data<AppState> {
        web::Data::new(AppState {
            songs: Mutex::new(songs()),
        })
    }

    #[actix_web::test]
    async fn test_home_returns_200() {
        let app = test::init_service(App::new().service(home)).await;
        let req = test::TestRequest::get().uri("/").send_request(&app).await;
        assert!(req.status().is_success());
    }

    #[actix_web::test]
    async fn test_home_returns_html_content_type() {
        let app = test::init_service(App::new().service(home)).await;
        let req = test::TestRequest::get().uri("/").send_request(&app).await;
        let content_type = req.headers().get("content-type").unwrap();
        assert!(content_type.to_str().unwrap().contains("text/html"));
    }

    #[actix_web::test]
    async fn test_unknown_route_returns_404() {
        let app = test::init_service(App::new().service(home)).await;
        let req = test::TestRequest::get()
            .uri("/nonexistent")
            .send_request(&app)
            .await;
        assert!(req.status().is_client_error());
    }

    #[actix_web::test]
    async fn test_update_song_returns_200() {
        let state = test_state();
        let app = test::init_service(App::new().app_data(state).service(update_song)).await;
        let req = test::TestRequest::put()
            .uri("/api/songs/1")
            .set_json(UpdateSong {
                title: "Blinding Lights (Remix)".to_string(),
                artist: "The Weeknd".to_string(),
                duration: "3:45".to_string(),
                url: "songs/blinding-lights-remix.mp3".to_string(),
            })
            .send_request(&app)
            .await;
        assert!(req.status().is_success());
    }

    #[actix_web::test]
    async fn test_update_song_returns_404_for_missing_id() {
        let state = test_state();
        let app = test::init_service(App::new().app_data(state).service(update_song)).await;
        let req = test::TestRequest::put()
            .uri("/api/songs/999")
            .set_json(UpdateSong {
                title: "Ghost Song".to_string(),
                artist: "Nobody".to_string(),
                duration: "0:00".to_string(),
                url: "songs/ghost.mp3".to_string(),
            })
            .send_request(&app)
            .await;
        assert_eq!(req.status(), 404);
    }
}
