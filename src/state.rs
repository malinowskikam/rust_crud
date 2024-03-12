use sqlx::postgres::PgPoolOptions;
use std::env::var as env_var;

#[derive(Clone)]
pub struct AppState {
    pub db_pool: sqlx::PgPool,
}

async fn create_db_pool() -> Result<sqlx::PgPool, Box<dyn std::error::Error + Sync + Send>> {
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&env_var("DATABASE_URL")?)
        .await?;

    Ok(pool)
}

impl AppState {
    pub async fn init() -> Result<AppState, Box<dyn std::error::Error + Sync + Send>> {
        let pool = create_db_pool().await?;
        Ok(AppState { db_pool: pool })
    }
}
