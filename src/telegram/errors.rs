use reqwest::Error as ReqwestError;
use serde_json::Error as SerdeError;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum TelegramError {
    #[error("Error getting response: {0}")]
    Reqwest(#[from] ReqwestError),

    #[error("Error deserializing response: {0}")]
    Serde(#[from] SerdeError),
}
