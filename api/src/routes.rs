use axum::{
    Router,
    http::{Method, StatusCode},
    routing::get,
};
use tower_http::{
    cors::CorsLayer,
    services::{ServeDir, ServeFile},
};
use whynot_errors::{AppError, JsonResult, json_ok};

use crate::models::{Account, AccountType};

async fn not_found() -> AppError {
    AppError {
        code: StatusCode::NOT_FOUND,
        message: "Not Found".to_string(),
    }
}

fn api_routes() -> Router {
    Router::new()
        .fallback(not_found)
        // TODO: Update this for prod builds to only allow the host/port that it's running from.
        .layer(CorsLayer::permissive().allow_methods([Method::GET, Method::POST]))
}

async fn h2() -> JsonResult<Account> {
    json_ok(Account {
        name: "hi".to_string(),
        account_type: AccountType::Asset,
        starting_balance: 0,
        last_clear: 0,
    })
}

pub fn routes() -> Router {
    Router::new()
        .route("/health", get(|| async { "ok" })) // The only GET
        .route("/h2", get(h2)) // The only GET
        .nest("/api", api_routes())
        .fallback_service(
            ServeDir::new("public").not_found_service(ServeFile::new("public/index.html")),
        )
}
