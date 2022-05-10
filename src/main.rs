use axum_playground::run;
use dotenv::dotenv;

#[tokio::main]
async fn main() {
    dotenv().ok();
    run().await.unwrap();
}
