use actix_files::Files;
use actix_web::{get, App, HttpResponse, HttpServer, Responder};
use serde::Serialize;

#[derive(Serialize)]
struct Song {
    id: u32,
    title: &'static str,
    artist: &'static str,
    duration_secs: u32,
    url: &'static str,
}

// NOTE: Static/mock song catalog. Replace with a real data source
// (database, external API, etc.) when persistence is introduced.
pub(crate) fn songs() -> Vec<Song> {
    vec![
        Song {
            id: 1,
            title: "SoundHelix Song 1",
            artist: "SoundHelix",
            duration_secs: 373,
            url: "https://www.soundhelix.com/examples/mp3/SoundHelix-Song-1.mp3",
        },
        Song {
            id: 2,
            title: "SoundHelix Song 2",
            artist: "SoundHelix",
            duration_secs: 368,
            url: "https://www.soundhelix.com/examples/mp3/SoundHelix-Song-2.mp3",
        },
        Song {
            id: 3,
            title: "SoundHelix Song 3",
            artist: "SoundHelix",
            duration_secs: 378,
            url: "https://www.soundhelix.com/examples/mp3/SoundHelix-Song-3.mp3",
        },
        Song {
            id: 4,
            title: "SoundHelix Song 4",
            artist: "SoundHelix",
            duration_secs: 294,
            url: "https://www.soundhelix.com/examples/mp3/SoundHelix-Song-4.mp3",
        },
        Song {
            id: 5,
            title: "SoundHelix Song 5",
            artist: "SoundHelix",
            duration_secs: 402,
            url: "https://www.soundhelix.com/examples/mp3/SoundHelix-Song-5.mp3",
        },
        Song {
            id: 6,
            title: "SoundHelix Song 6",
            artist: "SoundHelix",
            duration_secs: 385,
            url: "https://www.soundhelix.com/examples/mp3/SoundHelix-Song-6.mp3",
        },
        Song {
            id: 7,
            title: "SoundHelix Song 7",
            artist: "SoundHelix",
            duration_secs: 359,
            url: "https://www.soundhelix.com/examples/mp3/SoundHelix-Song-7.mp3",
        },
        Song {
            id: 8,
            title: "SoundHelix Song 8",
            artist: "SoundHelix",
            duration_secs: 313,
            url: "https://www.soundhelix.com/examples/mp3/SoundHelix-Song-8.mp3",
        },
        Song {
            id: 9,
            title: "SoundHelix Song 9",
            artist: "SoundHelix",
            duration_secs: 270,
            url: "https://www.soundhelix.com/examples/mp3/SoundHelix-Song-9.mp3",
        },
        Song {
            id: 10,
            title: "SoundHelix Song 10",
            artist: "SoundHelix",
            duration_secs: 361,
            url: "https://www.soundhelix.com/examples/mp3/SoundHelix-Song-10.mp3",
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
async fn list_songs() -> impl Responder {
    HttpResponse::Ok().json(songs())
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

    HttpServer::new(|| {
        App::new()
            .service(home)
            .service(list_songs)
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