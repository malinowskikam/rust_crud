mod api;
pub mod core;
pub mod dto;
pub mod models;
pub mod services;
pub mod util;

use actix_web::{
    web::{self, JsonConfig, PathConfig},
    App, HttpServer,
};

use api::user::users_service;
use core::{
    errors::{json_error_handler, path_error_handler},
    state::AppState,
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Sync + Send>> {
    let state = AppState::init().await?;

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(state.clone()))
            .app_data(PathConfig::default().error_handler(path_error_handler))
            .app_data(JsonConfig::default().error_handler(json_error_handler))
            .service(web::scope("/api").service(users_service()))
    })
    .bind(("127.0.0.1", 8000))?
    .run()
    .await?;

    Ok(())
}
