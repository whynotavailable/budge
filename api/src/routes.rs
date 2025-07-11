use axum::{
    Router,
    http::{Method, StatusCode},
    routing::get,
};
use tower_http::cors::CorsLayer;
use whynot_errors::AppError;

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

pub fn routes() -> Router {
    Router::new()
        .route("/health", get(|| async { "ok" })) // The only GET
        .nest("/api", api_routes())
}
