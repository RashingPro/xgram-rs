use crate::telegram_api::error::TelegramApiError;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum CommandHandlerError {
    #[error(transparent)]
    TelegramApiError(#[from] TelegramApiError)
}

pub type CommandHandlerResult = Result<(), CommandHandlerError>;
