pub mod http;

use crate::client::TelegramApiClient;
use crate::error::TelegramApiError;
use crate::types::update::Update;
use tokio::sync::mpsc;
use xgram_utils::types::BotConfigArc;

pub trait UpdateReceiver {
    fn spawn(self: Box<Self>) -> mpsc::Receiver<Result<Update, TelegramApiError>>;
}

pub type UpdateReceiverFactory =
    Box<dyn FnOnce(BotConfigArc, TelegramApiClient) -> Box<dyn UpdateReceiver>>;

pub macro update_receiver_factory($receiver:ty) {
    Box::new(
        |config: BotConfigArc, client: TelegramApiClient| -> Box<dyn UpdateReceiver> {
            Box::new(<$receiver>::new(config, client))
        }
    )
}
