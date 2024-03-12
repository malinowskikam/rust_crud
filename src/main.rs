mod api;
pub mod models;
pub mod state;
pub mod util;

use actix_web::{web, App, HttpServer};
use api::users::users_service;
use state::AppState;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Sync + Send>> {
    let state = AppState::init().await?;

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(state.clone()))
            .service(web::scope("/api").service(users_service()))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await?;

    Ok(())
}
