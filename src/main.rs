use actix_files::Files;
use actix_web::{get, App, HttpResponse, HttpServer, Responder};

#[get("/")]
async fn home() -> impl Responder {
    HttpResponse::Ok()
        .content_type("text/html")
        .body(include_str!("../templates/index.html"))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("Server running at http://127.0.0.1:8000");

    HttpServer::new(|| {
        App::new()
            .service(home)
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