use crate::{
    endpoints::{TelegramApiEndpoint, craft_default_url},
    types::update::Update
};
use serde::Serialize;
use serde_with::skip_serializing_none;

#[skip_serializing_none]
#[derive(Debug, Default, Serialize)]
pub struct GetUpdatesEndpoint {
    pub offset: Option<u32>,
    pub limit: Option<u8>,
    pub timeout: Option<u8>,
    pub allowed_updates: Option<Vec<String>>
}

impl TelegramApiEndpoint<Vec<Update>> for GetUpdatesEndpoint {
    fn craft_url(&self, token: &str, base_url: &str) -> String {
        craft_default_url(token, base_url, "getUpdates")
    }
}
