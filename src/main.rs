use actix_files::Files;
use actix_web::{delete, get, post, App, HttpResponse, HttpServer, Responder};
use serde::{Deserialize, Serialize};
use std::sync::{Arc, Mutex};

#[derive(Serialize, Clone)]
struct Song {
    id: u32,
    title: String,
    artist: String,
    duration: String,
    url: String,
}

#[derive(Deserialize)]
struct CreateSong {
    title: String,
    artist: String,
    duration: String,
    url: String,
}

struct AppState {
    songs: Mutex<Vec<Song>>,
}

#[derive(Serialize)]
struct ApiResponse<T: Serialize> {
    success: bool,
    data: T,
    message: String,
}

#[get("/")]
async fn home() -> impl Responder {
    HttpResponse::Ok()
        .content_type("text/html")
        .body(include_str!("../templates/index.html"))
}

#[get("/api/hello")]
async fn hello() -> impl Responder {
    let response = ApiResponse {
        success: true,
        data: serde_json::json!({"message": "Hello from RustTune"}),
        message: "Request Successful".to_string(),
    };
    HttpResponse::Ok().json(response)
}

#[get("/api/songs")]
async fn get_songs(data: actix_web::web::Data<Arc<AppState>>) -> impl Responder {
    let songs = data.songs.lock().unwrap();
    let response = ApiResponse {
        success: true,
        data: songs.clone(),
        message: "Songs retrieved successfully".to_string(),
    };
    HttpResponse::Ok().json(response)
}

#[post("/api/songs")]
async fn create_song(
    data: actix_web::web::Data<Arc<AppState>>,
    body: actix_web::web::Json<CreateSong>,
) -> impl Responder {
    let mut songs = data.songs.lock().unwrap();
    let new_song = Song {
        id: songs.len() as u32 + 1,
        title: body.title.clone(),
        artist: body.artist.clone(),
        duration: body.duration.clone(),
        url: body.url.clone(),
    };
    songs.push(new_song.clone());
    let response = ApiResponse {
        success: true,
        data: new_song,
        message: "Song created successfully".to_string(),
    };
    HttpResponse::Ok().json(response)
}

#[get("/api/songs/{id}")]
async fn get_song(
    data: actix_web::web::Data<Arc<AppState>>,
    path: actix_web::web::Path<u32>,
) -> impl Responder {
    let id = path.into_inner();
    let songs = data.songs.lock().unwrap();
    let song = songs.iter().find(|s| s.id == id);
    match song {
        Some(s) => {
            let response = ApiResponse {
                success: true,
                data: s.clone(),
                message: "Song retrieved successfully".to_string(),
            };
            HttpResponse::Ok().json(response)
        }
        None => {
            let response = ApiResponse {
                success: false,
                data: serde_json::json!({}),
                message: "Song not found".to_string(),
            };
            HttpResponse::NotFound().json(response)
        }
    }
}

#[delete("/api/songs/{id}")]
async fn delete_song(
    data: actix_web::web::Data<Arc<AppState>>,
    path: actix_web::web::Path<u32>,
) -> impl Responder {
    let id = path.into_inner();
    let mut songs = data.songs.lock().unwrap();
    let pos = songs.iter().position(|s| s.id == id);
    match pos {
        Some(i) => {
            songs.remove(i);
            let response = ApiResponse {
                success: true,
                data: serde_json::json!({}),
                message: "Song deleted successfully".to_string(),
            };
            HttpResponse::Ok().json(response)
        }
        None => {
            let response = ApiResponse {
                success: false,
                data: serde_json::json!({}),
                message: "Song not found".to_string(),
            };
            HttpResponse::NotFound().json(response)
        }
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("Server running at http://127.0.0.1:8000");

    let data = Arc::new(AppState {
        songs: Mutex::new(Vec::new()),
    });

    HttpServer::new(move || {
        App::new()
            .app_data(actix_web::web::Data::new(data.clone()))
            .service(home)
            .service(hello)
            .service(get_songs)
            .service(get_song)
            .service(create_song)
            .service(delete_song)
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

    #[actix_web::test]
    async fn test_home_returns_200() {
        let app = test::init_service(App::new().service(home)).await;
        let req = test::TestRequest::get().uri("/").send_request(&app).await;
        assert!(req.status().is_success());
    }

    #[actix_web::test]
    async fn test_hello_returns_consistent_format() {
        let app = test::init_service(App::new().service(hello)).await;
        let req = test::TestRequest::get()
            .uri("/api/hello")
            .send_request(&app)
            .await;
        assert!(req.status().is_success());
        let body: serde_json::Value = test::read_body_json(req).await;
        assert_eq!(body["success"], true);
        assert_eq!(body["message"], "Request Successful");
        assert!(body["data"].is_object());
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
    async fn test_get_songs_empty() {
        let data = Arc::new(AppState {
            songs: Mutex::new(Vec::new()),
        });
        let app = test::init_service(
            App::new()
                .app_data(actix_web::web::Data::new(data))
                .service(get_songs),
        )
        .await;
        let req = test::TestRequest::get()
            .uri("/api/songs")
            .send_request(&app)
            .await;
        assert!(req.status().is_success());
        let body: serde_json::Value = test::read_body_json(req).await;
        assert_eq!(body["success"], true);
    }

    #[actix_web::test]
    async fn test_create_song() {
        let data = Arc::new(AppState {
            songs: Mutex::new(Vec::new()),
        });
        let app = test::init_service(
            App::new()
                .app_data(actix_web::web::Data::new(data))
                .service(create_song),
        )
        .await;
        let req = test::TestRequest::post()
            .uri("/api/songs")
            .set_json(serde_json::json!({
                "title": "Blinding Lights",
                "artist": "The Weeknd",
                "duration": "3:20",
                "url": "songs/blinding-lights.mp3"
            }))
            .send_request(&app)
            .await;
        assert!(req.status().is_success());
        let body: serde_json::Value = test::read_body_json(req).await;
        assert_eq!(body["success"], true);
        assert_eq!(body["data"]["title"], "Blinding Lights");
    }

    #[actix_web::test]
    async fn test_get_song_not_found() {
        let data = Arc::new(AppState {
            songs: Mutex::new(Vec::new()),
        });
        let app = test::init_service(
            App::new()
                .app_data(actix_web::web::Data::new(data))
                .service(get_song),
        )
        .await;
        let req = test::TestRequest::get()
            .uri("/api/songs/999")
            .send_request(&app)
            .await;
        assert!(req.status().is_client_error());
    }

    #[actix_web::test]
    async fn test_delete_song() {
        let data = Arc::new(AppState {
            songs: Mutex::new(Vec::new()),
        });
        let app = test::init_service(
            App::new()
                .app_data(actix_web::web::Data::new(data))
                .service(create_song)
                .service(delete_song),
        )
        .await;
        test::TestRequest::post()
            .uri("/api/songs")
            .set_json(serde_json::json!({
                "title": "Blinding Lights",
                "artist": "The Weeknd",
                "duration": "3:20",
                "url": "songs/blinding-lights.mp3"
            }))
            .send_request(&app)
            .await;
        let req = test::TestRequest::delete()
            .uri("/api/songs/1")
            .send_request(&app)
            .await;
        assert!(req.status().is_success());
        let body: serde_json::Value = test::read_body_json(req).await;
        assert_eq!(body["success"], true);
    }
}
