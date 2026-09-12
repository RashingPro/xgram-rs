pub mod http;

use crate::error::TelegramApiError;
use crate::types::update::Update;
use tokio::sync::mpsc;

pub trait UpdateReceiver {
    fn spawn(self) -> mpsc::Receiver<Result<Update, TelegramApiError>>;
}
