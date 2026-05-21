use actix_web::{web, HttpRequest, HttpResponse, Responder};
use bcrypt::{hash, verify, DEFAULT_COST};
use chrono::Utc;
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};
use uuid::Uuid;

use crate::auth::models::{AuthResponse, Claims, LoginRequest, RegisterRequest, User, UserInfo};
use crate::db::DbPool;

fn get_secret() -> String {
    std::env::var("JWT_SECRET").unwrap_or_else(|_| "fallback_secret".to_string())
}

fn make_access_token(user_id: &str, username: &str) -> String {
    let exp = (Utc::now().timestamp() + 3600) as usize; // 1 hour
    let iat = Utc::now().timestamp() as usize;
    let claims = Claims {
        sub: user_id.to_string(),
        username: username.to_string(),
        exp,
        iat,
    };
    encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(get_secret().as_bytes()),
    )
    .expect("JWT encoding failed")
}

pub async fn register(
    pool: web::Data<DbPool>,
    body: web::Json<RegisterRequest>,
) -> impl Responder {
    let conn = pool.0.lock().unwrap();

    // Check if username exists
    let exists: bool = conn
        .query_row(
            "SELECT COUNT(*) FROM users WHERE username = ?1",
            [&body.username],
            |row| row.get::<_, i32>(0),
        )
        .unwrap_or(0)
        > 0;

    if exists {
        return HttpResponse::Conflict().json(serde_json::json!({
            "error": "Username already taken"
        }));
    }

    let password_hash = hash(&body.password, DEFAULT_COST).unwrap();
    let user_id = Uuid::new_v4().to_string();
    let created_at = Utc::now().to_rfc3339();

    conn.execute(
        "INSERT INTO users (id, username, password_hash, created_at) VALUES (?1, ?2, ?3, ?4)",
        [&user_id, &body.username, &password_hash, &created_at],
    )
    .unwrap();

    let access_token = make_access_token(&user_id, &body.username);
    let refresh_token = Uuid::new_v4().to_string();
    let refresh_expires = (Utc::now() + chrono::Duration::days(7)).to_rfc3339();

    conn.execute(
        "INSERT INTO refresh_tokens (id, user_id, token, expires_at) VALUES (?1, ?2, ?3, ?4)",
        [&Uuid::new_v4().to_string(), &user_id, &refresh_token, &refresh_expires],
    )
    .unwrap();

    HttpResponse::Created().json(AuthResponse {
        access_token,
        refresh_token,
        token_type: "Bearer".to_string(),
    })
}

pub async fn login(
    pool: web::Data<DbPool>,
    body: web::Json<LoginRequest>,
) -> impl Responder {
    let conn = pool.0.lock().unwrap();

    let result = conn.query_row(
        "SELECT id, username, password_hash, created_at FROM users WHERE username = ?1",
        [&body.username],
        |row| {
            Ok(User {
                id: row.get(0)?,
                username: row.get(1)?,
                password_hash: row.get(2)?,
                created_at: row.get(3)?,
            })
        },
    );

    match result {
        Ok(user) => {
            if verify(&body.password, &user.password_hash).unwrap_or(false) {
                let access_token = make_access_token(&user.id, &user.username);
                let refresh_token = Uuid::new_v4().to_string();
                let refresh_expires = (Utc::now() + chrono::Duration::days(7)).to_rfc3339();

                conn.execute(
                    "INSERT INTO refresh_tokens (id, user_id, token, expires_at) VALUES (?1, ?2, ?3, ?4)",
                    [&Uuid::new_v4().to_string(), &user.id, &refresh_token, &refresh_expires],
                ).unwrap();

                HttpResponse::Ok().json(AuthResponse {
                    access_token,
                    refresh_token,
                    token_type: "Bearer".to_string(),
                })
            } else {
                HttpResponse::Unauthorized().json(serde_json::json!({
                    "error": "Invalid credentials"
                }))
            }
        }
        Err(_) => HttpResponse::Unauthorized().json(serde_json::json!({
            "error": "Invalid credentials"
        })),
    }
}

pub async fn me(req: HttpRequest) -> impl Responder {
    let auth_header = req.headers().get("Authorization");

    match auth_header {
        Some(val) => {
            let token_str = val.to_str().unwrap_or("").replace("Bearer ", "");
            let secret = get_secret();

            match decode::<Claims>(
                &token_str,
                &DecodingKey::from_secret(secret.as_bytes()),
                &Validation::default(),
            ) {
                Ok(token_data) => {
                    HttpResponse::Ok().json(UserInfo {
                        id: token_data.claims.sub,
                        username: token_data.claims.username,
                        created_at: "".to_string(), // can enrich from DB if needed
                    })
                }
                Err(_) => HttpResponse::Unauthorized().json(serde_json::json!({
                    "error": "Invalid or expired token"
                })),
            }
        }
        None => HttpResponse::Unauthorized().json(serde_json::json!({
            "error": "Missing Authorization header"
        })),
    }
}