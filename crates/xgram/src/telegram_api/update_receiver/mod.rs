pub mod long_polling;

use crate::{
    telegram_api::{client::TelegramApiClient, error::TelegramApiError, types::update::Update},
    utils::types::InnerConfigArc
};
use tokio::sync::mpsc;

pub trait UpdateReceiver: Sync + Send + 'static {
    fn new(config: InnerConfigArc, client: TelegramApiClient) -> Self;

    fn spawn(self) -> mpsc::Receiver<Result<Update, TelegramApiError>>;
}
