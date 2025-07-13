use models::Account;
use routes::routes;
use sqlx::{postgres::PgPoolOptions, query_as};
use whynot_errors::{SetupError, SetupResult};

mod models;
mod routes;

pub async fn run(conn_str: String, host: String) -> SetupResult {
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(conn_str.as_str())
        .await
        .map_err(SetupError::new)?;

    let app = routes();

    let listener = tokio::net::TcpListener::bind(host)
        .await
        .map_err(SetupError::new)?;

    axum::serve(listener, app).await.map_err(SetupError::new)?;
    Ok(())
}
