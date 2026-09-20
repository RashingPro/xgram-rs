pub mod long_polling;

use crate::{client::TelegramApiClient, error::TelegramApiError, types::update::Update};
use tokio::sync::mpsc;
use xgram_utils::types::InnerConfigArc;

pub trait UpdateReceiver {
    fn new(config: InnerConfigArc, client: TelegramApiClient) -> Self;

    fn spawn(self) -> mpsc::Receiver<Result<Update, TelegramApiError>>;
}
