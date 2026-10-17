//! Telegram commands related logic.

pub mod context;
pub mod error;

use crate::bot::command::{context::CommandContext, error::CommandHandlerResult};
use std::pin::Pin;

pub type CommandHandlerFuture = Pin<Box<dyn Future<Output = CommandHandlerResult> + Send>>;
pub type CommandHandler = fn(CommandContext) -> CommandHandlerFuture;
