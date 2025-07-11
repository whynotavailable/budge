use routes::routes;
use whynot_errors::{SetupError, SetupResult};

mod models;
mod routes;

pub async fn run(conn_str: String) -> SetupResult {
    // build our application with a single route
    let app = routes();

    // run our app with hyper, listening globally on port 3000
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000")
        .await
        .map_err(SetupError::new)?;

    axum::serve(listener, app).await.map_err(SetupError::new)?;
    Ok(())
}
