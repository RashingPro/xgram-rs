//! Powerful yet blazingly fast Telegram Bot API framework written in Rust,
//! featuring advanced abstractions and Tokio-powered concurrency.

pub mod bot;
pub mod telegram_api;
pub mod utils;

pub use xgram_proc as proc;

/// Exports the most useful items. Recommended to import using a wildcard
/// syntax: `use xgram::prelude::*`
pub mod prelude {
    pub use crate::{
        bot::{
            Bot,
            command::{context::CommandContext, error::CommandHandlerResult}
        },
        proc::*,
        utils::config::Config
    };
}
