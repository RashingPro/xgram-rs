use serde::de::DeserializeOwned;
use serde::{Deserialize, Serialize};
use std::fmt::Debug;

pub mod get_updates;
pub mod send_message;

#[derive(Deserialize)]
pub struct TelegramApiResponse<R> {
    pub ok: bool,
    pub description: Option<String>,
    pub result: Option<R>
}

pub trait TelegramApiEndpoint<R>: Serialize + Debug
where
    R: DeserializeOwned
{
    fn craft_url(&self, token: &str, base_url: &str) -> String;
}

pub fn craft_default_url(token: &str, base_url: &str, endpoint: &str) -> String {
    format!("{}bot{}/{}", base_url, token, endpoint)
}
