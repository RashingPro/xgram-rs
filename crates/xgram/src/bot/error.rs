use crate::command::error::CommandHandlerError;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum BotError {
    #[error(transparent)]
    CommandHandlerError(#[from] CommandHandlerError),
    #[error("Unreachable code was reached. This is a bug - please report!")]
    UnreachableCodeReached
}

pub type BotResult = Result<!, BotError>;
