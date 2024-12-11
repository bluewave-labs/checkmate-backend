use axum::http::StatusCode;
use axum::{routing::get, Router};
use dotenv::dotenv;

use core::dal;
use core::state::AppGlobalState;
use lambda_http::{run, tracing, Error};
use std::env::set_var;
use routes::auth;

mod core;
mod routes;


async fn health_check() -> (StatusCode, String) {
    let health = true;
    match health {
        true => (StatusCode::OK, "Healthy!".to_string()),
        false => (
            StatusCode::INTERNAL_SERVER_ERROR,
            "Not healthy!".to_string(),
        ),
    }
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    tracing::init_default_subscriber();
    dotenv().ok();
    let db_pool = dal::pool::init_db().await;
    let state = AppGlobalState { pool: db_pool };

    let app = Router::new()
        .route("/health", get(health_check))
        .nest("/auth", auth::router::router())
        .with_state(state);

    tracing::info!("App is started");

    run(app).await
}
