use api::run;
use whynot_errors::SetupResult;

#[tokio::main]
async fn main() -> SetupResult {
    return run().await;
}
