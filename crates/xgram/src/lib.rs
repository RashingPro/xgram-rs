//! Powerful yet blazingly fast Telegram Bot API framework written in Rust,
//! featuring advanced abstractions and Tokio-powered concurrency.

pub mod bot;

pub mod command;

pub use xgram_proc as proc;
pub use xgram_telegram_api as telegram_api;
pub use xgram_utils as utils;

/// Exports the most useful items. Recommended to import using a wildcard
/// syntax: `use xgram::prelude::*`
pub mod prelude {
    pub use crate::{
        bot::{Bot, config::BotConfig},
        command::{context::CommandContext, error::CommandHandlerResult},
        proc::*
    };
}
