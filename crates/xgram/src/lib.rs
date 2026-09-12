pub mod bot;
pub mod command;

pub use xgram_proc as proc;
pub use xgram_telegram_api as telegram_api;
pub use xgram_utils as utils;

pub mod prelude {
    pub use crate::bot::Bot;
    pub use crate::command::context::CommandContext;
    pub use crate::proc::*;
    pub use crate::utils::config::BotConfig;
}
