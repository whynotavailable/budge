use routes::routes;
use whynot_errors::{SetupError, SetupResult};

mod models;
mod routes;

pub async fn run(conn_str: String, host: String) -> SetupResult {
    let app = routes();

    let listener = tokio::net::TcpListener::bind(host)
        .await
        .map_err(SetupError::new)?;

    axum::serve(listener, app).await.map_err(SetupError::new)?;
    Ok(())
}
