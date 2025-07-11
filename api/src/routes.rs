use axum::{Router, http::StatusCode, routing::get};
use whynot_errors::AppError;

async fn not_found() -> AppError {
    AppError {
        code: StatusCode::NOT_FOUND,
        message: "Not Found".to_string(),
    }
}

fn api_routes() -> Router {
    Router::new().fallback(not_found)
}

pub fn routes() -> Router {
    Router::new()
        .route("/health", get(|| async { "ok" })) // The only GET
        .nest("/api", api_routes())
}
