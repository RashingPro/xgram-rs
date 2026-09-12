pub mod context;

use crate::command::context::CommandContext;
use std::pin::Pin;

pub type CommandHandlerFuture = Pin<Box<dyn Future<Output = ()> + Send>>;
pub type CommandHandler = Box<dyn Fn(CommandContext) -> CommandHandlerFuture + Send + Sync>;
