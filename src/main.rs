use actix_files::Files;
use actix_web::{get, App, HttpResponse, HttpServer, Responder};
use serde::Serialize;

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

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("Server running at http://127.0.0.1:8000");

    HttpServer::new(|| {
        App::new()
            .service(home)
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
    async fn test_hello_returns_consistent_format() {
        let app = test::init_service(App::new().service(hello)).await;
        let req = test::TestRequest::get().uri("/api/hello").send_request(&app).await;
        assert!(req.status().is_success());
        let body: serde_json::Value = test::read_body_json(req).await;
        assert_eq!(body["success"], true);
        assert_eq!(body["message"], "Request Successful");
        assert!(body["data"].is_object());
    }
}