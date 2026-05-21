use actix_web::{dev::Payload, error::ErrorUnauthorized, FromRequest, HttpRequest};
use std::future::{ready, Ready};
use jsonwebtoken::{decode, DecodingKey, Validation};
use crate::auth::models::Claims;

pub struct AuthenticatedUser {
    pub user_id: String,
    pub username: String,
}

impl FromRequest for AuthenticatedUser {
    type Error = actix_web::Error;
    type Future = Ready<Result<Self, Self::Error>>;

    fn from_request(req: &HttpRequest, _: &mut Payload) -> Self::Future {
        let secret = std::env::var("JWT_SECRET").unwrap_or_else(|_| "fallback_secret".to_string());

        let token = req
            .headers()
            .get("Authorization")
            .and_then(|v| v.to_str().ok())
            .and_then(|v| v.strip_prefix("Bearer "))
            .map(|s| s.to_string());

        match token {
            Some(t) => {
                match decode::<Claims>(
                    &t,
                    &DecodingKey::from_secret(secret.as_bytes()),
                    &Validation::default(),
                ) {
                    Ok(data) => ready(Ok(AuthenticatedUser {
                        user_id: data.claims.sub,
                        username: data.claims.username,
                    })),
                    Err(_) => ready(Err(ErrorUnauthorized("Invalid or expired token"))),
                }
            }
            None => ready(Err(ErrorUnauthorized("Missing token"))),
        }
    }
}