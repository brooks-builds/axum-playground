use std::net::SocketAddr;

use axum::{
    routing::{get, post},
    Router,
};
use eyre::Result;

use crate::{
    games::create_game::create_game,
    stripe::{handle_single_stripe_payment, stripe_webhook},
};

mod api;
mod games;
pub mod stripe;

pub async fn run() -> Result<()> {
    tracing_subscriber::fmt::init();

    let app = Router::new()
        .route("/checkout", post(handle_single_stripe_payment))
        .route("/stripe-webhook", post(stripe_webhook))
        .route("/games", post(create_game));

    let address = SocketAddr::from(([127, 0, 0, 1], 8000));
    tracing::debug!("Listening on {address}");
    axum::Server::bind(&address)
        .serve(app.into_make_service())
        .await?;

    Ok(())
}
