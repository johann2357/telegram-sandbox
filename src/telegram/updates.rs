use std::collections::HashMap;

use reqwest;
use serde::{Deserialize, Serialize};

use super::errors::TelegramError;

#[derive(Debug, Deserialize, Serialize)]
pub struct Update {
    pub update_id: i64,
    pub message: Message,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Message {
    pub message_id: i64,
    pub from: User,
    pub chat: Chat,
    pub date: i64,
    pub text: Option<String>,
    pub entities: Option<Vec<Entity>>,
    pub forward_from: Option<User>,
    pub forward_date: Option<i64>,
    pub photo: Option<Vec<Photo>>,
    pub caption: Option<String>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct User {
    pub id: i64,
    pub is_bot: bool,
    pub first_name: String,
    pub username: Option<String>,
    pub language_code: Option<String>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Chat {
    pub id: i64,
    pub title: String,
    pub r#type: String,
    pub all_members_are_administrators: bool,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Entity {
    pub offset: i64,
    pub length: i64,
    pub r#type: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Photo {
    pub file_id: String,
    pub file_unique_id: String,
    pub file_size: Option<i64>,
    pub width: i64,
    pub height: i64,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct GetUpdates {
    pub ok: bool,
    pub result: Vec<Update>,
}

pub async fn get_updates(
    offset: i64,
    limit: i64,
    timeout: i64,
    token: &String,
) -> Result<GetUpdates, TelegramError> {
    let url = format!("https://api.telegram.org/bot{}/getUpdates", token);

    let mut params = HashMap::new();
    params.insert("offset", offset);
    params.insert("limit", limit);
    params.insert("timeout", timeout);

    let client = reqwest::Client::new();
    let response = client
        .get(&url)
        .query(&params)
        .send()
        .await
        .map_err(TelegramError::Reqwest)?;

    let response_body = response.text().await.map_err(TelegramError::Reqwest)?;

    let updates: GetUpdates = serde_json::from_str(&response_body).map_err(TelegramError::Serde)?;

    return Ok(updates);
}
