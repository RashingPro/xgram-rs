use crate::endpoints::{TelegramApiEndpoint, TelegramApiResponse};
use crate::error::TelegramApiError;
use log::trace;
use reqwest::Client;
use serde::de::DeserializeOwned;
use std::sync::Arc;
use xgram_utils::types::BotConfigArc;

#[derive(Clone)]
pub struct TelegramApiClient {
    config: BotConfigArc,
    token: Arc<String>,
    client: Client
}

impl TelegramApiClient {
    pub fn new(config: BotConfigArc, token: Arc<String>) -> Self {
        Self {
            config,
            token,
            client: Client::new()
        }
    }

    pub async fn make_request<R, T>(&self, endpoint: &T) -> Result<R, TelegramApiError>
    where
        R: DeserializeOwned,
        T: TelegramApiEndpoint<R>
    {
        trace!("Making request {:#?}", endpoint);
        let request = self
            .client
            .post(endpoint.craft_url(&self.token, self.config.base_api_url))
            .json(endpoint)
            .send();
        let response: TelegramApiResponse<R> = request.await?.json().await?;
        if !response.ok {
            return Err(TelegramApiError::NotSuccess(
                response
                    .description
                    .unwrap_or(String::from("no description present in response"))
            ));
        }
        response.result.ok_or_else(|| {
            TelegramApiError::NotSuccess(
                response
                    .description
                    .unwrap_or(String::from("no description present in response"))
            )
        })
    }
}
