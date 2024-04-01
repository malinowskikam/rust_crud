use crate::{
    core::{errors::ApiError, state::AppState},
    dto::login::LoginPayload,
    models::user::User,
    services::login::login,
};
use actix_web::{http::header::AUTHORIZATION, web::Data, FromRequest, Result};
use argon2::{
    password_hash::{rand_core::OsRng, PasswordHash, PasswordHasher, PasswordVerifier, SaltString},
    Argon2,
};
use base64::{engine::general_purpose::STANDARD as b64, Engine as _};
use futures::Future;
use sqlx::PgPool;
use std::pin::Pin;

impl FromRequest for User {
    type Error = ApiError;
    type Future = Pin<Box<dyn Future<Output = Result<Self, Self::Error>>>>;

    fn from_request(req: &actix_web::HttpRequest, _: &mut actix_web::dev::Payload) -> Self::Future {
        let headers = req.headers().clone();
        let pool = req
            .app_data::<Data<AppState>>()
            .map(|data| data.db_pool.clone());
        let pool = match pool {
            Some(pool) => pool,
            None => return Box::pin(async { Err(ApiError::InternalServerError) }),
        };

        Box::pin(async move {
            let auth = headers
                .get(AUTHORIZATION)
                .ok_or(ApiError::Unauthorized)?
                .to_str()
                .map_err(|_| ApiError::Unauthorized)?;

            if let Some(user) = header_auth(&pool, auth).await {
                return Ok(user);
            }
            Err(ApiError::Unauthorized)
        })
    }
}

async fn header_auth(pool: &PgPool, auth: &str) -> Option<User> {
    let mut split = auth.splitn(2, ' ');
    let token_type = split.next()?;
    let token = split.next()?;

    if token_type == "Basic" {
        let decoded = b64.decode(token).ok()?;
        let decoded = String::from_utf8(decoded).ok()?;
        let mut split = decoded.splitn(2, ':');
        let username = split.next()?;
        let password = split.next()?;

        let user = login(
            pool,
            &LoginPayload {
                username: username.to_string(),
                password: password.to_string(),
            },
        )
        .await
        .ok()?;
        return Some(user);
    };

    None
}

pub fn generate_password_hash(password: &str) -> Result<String> {
    let password_salt = SaltString::generate(&mut OsRng);
    let argon2 = Argon2::default();
    let password_hash = argon2
        .hash_password(password.as_bytes(), &password_salt)
        .map_err(|_| ApiError::InternalServerError)?
        .to_string();

    Ok(password_hash)
}

pub fn verify_password(password: &str, password_hash: &str) -> Result<()> {
    let parsed_hash =
        PasswordHash::new(password_hash).map_err(|_| ApiError::InternalServerError)?;
    Argon2::default()
        .verify_password(password.as_bytes(), &parsed_hash)
        .map_err(|_| ApiError::Unauthorized)?;
    Ok(())
}
