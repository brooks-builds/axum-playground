use axum_playground::run;
use dotenv::dotenv;
use dotenv_codegen::dotenv;

#[tokio::main]
async fn main() {
    dotenv().ok();
    let sentry_uri = dotenv!("SENTRY_URI");
    let _guard = sentry::init((
        sentry_uri,
        sentry::ClientOptions {
            attach_stacktrace: true,
            ..Default::default()
        },
    ));
    run().await.unwrap();
}
