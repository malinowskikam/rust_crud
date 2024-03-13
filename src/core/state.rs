use sqlx::postgres::PgPoolOptions;
use std::env::var as env_var;

use crate::models::user::User;

#[derive(Clone)]
pub struct AppState {
    pub db_pool: sqlx::PgPool,
    pub user: Option<User>,
}

async fn create_db_pool() -> Result<sqlx::PgPool, Box<dyn std::error::Error + Sync + Send>> {
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&env_var("DATABASE_URL").map_err(|_| "DATABASE_URL is not set")?)
        .await?;

    Ok(pool)
}

impl AppState {
    pub async fn init() -> Result<AppState, Box<dyn std::error::Error + Sync + Send>> {
        let pool = create_db_pool().await?;
        Ok(AppState {
            db_pool: pool,
            user: None,
        })
    }
}
