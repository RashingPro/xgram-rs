//! Telegram commands related logic.

pub mod context;
pub mod error;

use crate::command::{context::CommandContext, error::CommandHandlerResult};
use std::pin::Pin;

pub type CommandHandlerFuture = Pin<Box<dyn Future<Output = CommandHandlerResult> + Send>>;
pub type CommandHandler = Box<dyn Fn(CommandContext) -> CommandHandlerFuture + Send + Sync>;
