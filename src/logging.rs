use axum::response::IntoResponse;
use chrono::prelude::*;
use dotenv_codegen::dotenv;
use eyre::{bail, Result};
use reqwest::StatusCode;
use serde_json::json;

pub async fn logging() -> impl IntoResponse {
    dbg!("logging to seq");
    let logging_uri = dotenv!("LOGGING_URI");
    let client = reqwest::Client::new();
    let now = Local::now();
    if let Err(error) = i_error() {
        let log = Log {
            level: LogLevel::Error,
            message: "meeeeeow".to_owned(),
            stack_trace: Some(format!("{:?}", error)),
        };
        client
            .post(logging_uri)
            .json(&json!({
              "@t": now.to_string(),
              "@l": log.level.to_string(),
              "@m": log.message,
              "@x": log.stack_trace
            }))
            .header("X-Seq-ApiKey", dotenv!("LOGGING_KEY"))
            .send()
            .await
            .unwrap();
    }

    StatusCode::OK
}

pub struct Log {
    pub level: LogLevel,
    pub message: String,
    pub stack_trace: Option<String>,
}

pub enum LogLevel {
    Error,
    Warning,
    Meow,
}

impl ToString for LogLevel {
    fn to_string(&self) -> String {
        match self {
            LogLevel::Error => "Error".to_owned(),
            LogLevel::Warning => "Warning".to_owned(),
            LogLevel::Meow => "Meow".to_owned(),
        }
    }
}

fn i_error() -> Result<()> {
    bail!("I am an error, hi! :)");
}
