use reqwest;
use serde::{Deserialize, Serialize};

use super::errors::TelegramError;

#[derive(Debug, Deserialize, Serialize)]
pub struct File {
    pub file_id: String,
    pub file_unique_id: String,
    pub file_size: u64,
    pub file_path: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct GetFile {
    pub ok: bool,
    pub result: File,
}

pub async fn get_file(file_id: String, token: &String) -> Result<GetFile, TelegramError> {
    let url = format!(
        "https://api.telegram.org/bot{}/getFile?file_id={}",
        token, file_id
    );
    let response = reqwest::get(url).await.map_err(TelegramError::Reqwest)?;

    let response_text = response.text().await.map_err(TelegramError::Reqwest)?;

    let file: GetFile = serde_json::from_str(&response_text).map_err(TelegramError::Serde)?;

    return Ok(file);
}
