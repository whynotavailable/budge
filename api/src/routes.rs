use axum::{Router, routing::get};

fn api_routes() -> Router {
    Router::new()
}

pub fn routes() -> Router {
    Router::new()
        .route("/health", get(|| async { "ok" })) // The only GET
        .nest("/api", api_routes())
}
