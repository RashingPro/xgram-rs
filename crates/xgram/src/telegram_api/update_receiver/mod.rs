pub mod long_polling;

use crate::{
    telegram_api::{client::TelegramApiClient, error::TelegramApiError, types::update::Update},
    utils::types::ConfigArc
};
use tokio::sync::mpsc;

pub trait UpdateReceiver: Sync + Send + 'static {
    fn new(config: ConfigArc, client: TelegramApiClient) -> Self;

    fn spawn(self) -> mpsc::Receiver<Result<Update, TelegramApiError>>;
}
