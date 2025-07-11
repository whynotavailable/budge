use std::{env, str};

use api::run;
use whynot_errors::{SetupError, SetupResult};

fn require_env(key: &'static str) -> Result<String, SetupError> {
    env::var(key).map_err(|_| SetupError::new(format!("ENV '{key}' missing")))
}

#[tokio::main]
async fn main() -> SetupResult {
    let conn_str = require_env("CONN_STR")?;
    let host = require_env("HOST")?;

    return run(conn_str, host).await;
}
