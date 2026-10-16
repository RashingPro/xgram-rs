use crate::bot::command::error::CommandHandlerError;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum BotError {
    #[error(transparent)]
    CommandHandlerError(#[from] CommandHandlerError),
    #[error("The main loop was terminated due to an error or a bug.")]
    MainLoopTerminated
}

pub type BotResult = Result<!, BotError>;
