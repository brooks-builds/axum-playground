use anyhow::{bail, Result};
use axum::response::IntoResponse;
use reqwest::StatusCode;
use sentry::{capture_error, capture_message, integrations::anyhow::capture_anyhow, Level};

pub async fn panic_for_sentry() -> impl IntoResponse {
    if let Err(error) = fn_with_anyhow_error() {
        capture_anyhow(&error);
        capture_message(
            "Had an error running the function that always errors",
            Level::Debug,
        );
    }
    StatusCode::INTERNAL_SERVER_ERROR
}

fn fn_that_errors() -> Result<()> {
    bail!("I am bailing");
}

fn fn_with_anyhow_error() -> Result<()> {
    Err(anyhow::anyhow!("I am erroring with anyhow!"))
}
