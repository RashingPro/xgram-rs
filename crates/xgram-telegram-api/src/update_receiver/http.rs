use crate::client::TelegramApiClient;
use crate::endpoints::get_updates::GetUpdatesEndpoint;
use crate::error::TelegramApiError;
use crate::types::update::Update;
use crate::update_receiver::UpdateReceiver;
use log::{error, warn};
use std::cmp::max;
use std::time::Duration;
use tokio::sync::mpsc;
use tokio::sync::mpsc::Receiver;
use tokio::sync::mpsc::error::TrySendError;
use tokio::time::sleep;
use xgram_utils::types::InnerConfigArc;

pub struct HttpUpdateReceiver {
    config: InnerConfigArc,
    client: TelegramApiClient
}

impl HttpUpdateReceiver {
    pub fn new(config: InnerConfigArc, client: TelegramApiClient) -> Self {
        Self { config, client }
    }
}

impl UpdateReceiver for HttpUpdateReceiver {
    fn spawn(self: Box<Self>) -> Receiver<Result<Update, TelegramApiError>> {
        let (tx, rx) = mpsc::channel(self.config.update_buffer_capacity);

        tokio::spawn(async move {
            let mut offset = 0;
            let mut retry_backoff_multiplier = 0;

            let mut did_warned_channel_is_full = false;

            loop {
                let response = self
                    .client
                    .make_request(&GetUpdatesEndpoint {
                        offset: Some(offset),
                        timeout: Some(self.config.http_updates_polling_timeout),
                        ..Default::default()
                    })
                    .await;

                match response {
                    Ok(updates) => {
                        retry_backoff_multiplier = 0;
                        let mut m = 0_u32;
                        for update in updates {
                            m = max(m, update.update_id);
                            let update = Ok(update);
                            if let Err(err) = tx.try_send(update) {
                                match err {
                                    TrySendError::Full(update) => {
                                        if !did_warned_channel_is_full {
                                            warn!(target: "update_receiver", "Update buffer is full. This is a soft warning and will not be emitted any more. See doc comment under `xgram::bot::config::BotConfig::update_buffer_capacity`");
                                            did_warned_channel_is_full = true;
                                        }

                                        if tx.send(update).await.is_err() {
                                            break;
                                        }
                                    }
                                    TrySendError::Closed(_) => {
                                        break;
                                    }
                                }
                            }
                        }
                        offset = m + 1;
                    }
                    Err(err) => {
                        if tx.send(Err(err)).await.is_err() {
                            break;
                        }
                        retry_backoff_multiplier += 1;
                        sleep(Duration::from_secs(retry_backoff_multiplier)).await;
                    }
                };
            }
            error!(
                target: "update_receiver",
                "Update receiver loop finished, but it wasn't supposed to! It may be a bug - \
                 consider reporting."
            );
        });

        rx
    }
}
