use axum_playground::run;

#[tokio::main]
async fn main() {
    dotenv::dotenv().ok();
    run().await.unwrap();
}
