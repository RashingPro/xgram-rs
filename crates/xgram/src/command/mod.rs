pub mod context;
pub mod error;

use crate::command::context::CommandContext;
use crate::command::error::CommandHandlerResult;
use std::pin::Pin;

pub type CommandHandlerFuture = Pin<Box<dyn Future<Output = CommandHandlerResult> + Send>>;
pub type CommandHandler = Box<dyn Fn(CommandContext) -> CommandHandlerFuture + Send + Sync>;
