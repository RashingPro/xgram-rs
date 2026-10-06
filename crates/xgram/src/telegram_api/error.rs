use std::fmt::Debug;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum TelegramApiError {
    #[error("Reqwest error: {0}")]
    Reqwest(#[from] reqwest::Error),
    #[error("SerdeJson error: {0}")]
    SerdeJson(#[from] serde_json::Error),
    #[error("Not success HTTP code received: {0}")]
    NotSuccess(String)
}
