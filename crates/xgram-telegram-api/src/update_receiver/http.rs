use crate::client::TelegramApiClient;
use crate::endpoints::get_updates::GetUpdatesEndpoint;
use crate::error::TelegramApiError;
use crate::types::update::Update;
use crate::update_receiver::UpdateReceiver;
use std::cmp::max;
use std::time::Duration;
use tokio::sync::mpsc;
use tokio::sync::mpsc::Receiver;
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
                            if tx.send(Ok(update)).await.is_err() {
                                break;
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
        });

        rx
    }
}
