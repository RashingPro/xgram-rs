use thiserror::Error;
use xgram_telegram_api::error::TelegramApiError;

#[derive(Debug, Error)]
pub enum CommandHandlerError {
    #[error(transparent)]
    TelegramApiError(#[from] TelegramApiError)
}

pub type CommandHandlerResult = Result<(), CommandHandlerError>;
