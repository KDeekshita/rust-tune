use actix_files::Files;
use actix_web::{get, web, App, HttpResponse, HttpServer, Responder};

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

async fn not_found() -> impl Responder {
    HttpResponse::NotFound()
        .content_type("text/html")
        .body(include_str!("../templates/404.html"))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("Server running at http://127.0.0.1:8000");

    HttpServer::new(|| {
        App::new()
            .service(home)
            .service(hello)
            .service(Files::new("/static", "./static"))
            .default_service(web::route().to(not_found))
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

        let req = test::TestRequest::get()
            .uri("/")
            .send_request(&app)
            .await;

        assert!(req.status().is_success());
    }

    #[actix_web::test]
    async fn test_home_returns_html_content_type() {
        let app = test::init_service(App::new().service(home)).await;

        let req = test::TestRequest::get()
            .uri("/")
            .send_request(&app)
            .await;

        let content_type = req.headers().get("content-type").unwrap();

        assert!(content_type.to_str().unwrap().contains("text/html"));
    }

    #[actix_web::test]
    async fn test_unknown_route_returns_404() {
        let app = test::init_service(
            App::new()
                .service(home)
                .default_service(web::route().to(not_found)),
        )
        .await;

        let req = test::TestRequest::get()
            .uri("/nonexistent")
            .send_request(&app)
            .await;

        assert!(req.status().is_client_error());
    }
}