pub mod http;

use crate::client::TelegramApiClient;
use crate::error::TelegramApiError;
use crate::types::update::Update;
use tokio::sync::mpsc;
use xgram_utils::types::InnerConfigArc;

pub trait UpdateReceiver {
    fn new(config: InnerConfigArc, client: TelegramApiClient) -> Self;

    fn spawn(self) -> mpsc::Receiver<Result<Update, TelegramApiError>>;
}
