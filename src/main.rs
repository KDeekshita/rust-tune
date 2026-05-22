use actix_files::Files;
use actix_web::{get, web, App, HttpResponse, HttpServer, Responder};
use serde::Serialize;
use std::env;

#[derive(Serialize, Clone)]
struct Song {
    id: u32,
    title: String,
    artist: String,
    duration: String,
    url: String,
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

#[get("/")]
async fn home() -> impl Responder {
    HttpResponse::Ok()
        .content_type("text/html")
        .body(include_str!("../templates/index.html"))
}

#[get("/api/songs")]
async fn list_songs(data: web::Data<Vec<Song>>) -> impl Responder {
    web::Json(data.get_ref().clone())
}

#[derive(Serialize)]
struct HelloResponse {
    message: String,
    status: String,
}

#[get("/api/hello")]
async fn hello() -> impl Responder {
    web::Json(HelloResponse {
        message: "Hello from RustTune!".into(),
        status: "ok".into(),
    })
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenvy::dotenv().ok();
    let host = env::var("HOST").unwrap_or_else(|_| "127.0.0.1".to_string());
    let port: u16 = env::var("PORT")
        .unwrap_or_else(|_| "8000".to_string())
        .parse()
        .expect("PORT must be a valid u16");

    println!("Server running at http://{}:{}", host, port);

    let song_data = web::Data::new(songs());

    HttpServer::new(move || {
        App::new()
            .app_data(song_data.clone())
            .service(home)
            .service(list_songs)
            .service(hello)
            .service(Files::new("/static", "./static"))
    })
    .bind((host.as_str(), port))?
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
    async fn test_home_returns_html_content_type() {
        let app = test::init_service(App::new().service(home)).await;
        let req = test::TestRequest::get().uri("/").send_request(&app).await;
        let content_type = req.headers().get("content-type").unwrap();
        assert!(content_type.to_str().unwrap().contains("text/html"));
    }

    #[actix_web::test]
    async fn test_unknown_route_returns_404() {
        let app = test::init_service(App::new().service(home)).await;
        let req = test::TestRequest::get().uri("/nonexistent").send_request(&app).await;
        assert!(req.status().is_client_error());
    }
}