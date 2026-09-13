use xgram_telegram_api::update_receiver::http::HttpUpdateReceiver;
use xgram_telegram_api::update_receiver::{UpdateReceiverFactory, update_receiver_factory};
use xgram_utils::config::BotConfig;

pub struct XgramConfig {
    pub bot_config: BotConfig,
    /// Allows you to choose `UpdateReceiver`. You can use
    /// `update_receiver_factory!(UpdateReceiverStruct)` macro. Defaults to
    /// `update_receiver_factory!(HttpUpdateReceiver)`.
    pub update_receiver: UpdateReceiverFactory
}

impl Default for XgramConfig {
    fn default() -> Self {
        Self {
            bot_config: Default::default(),
            update_receiver: update_receiver_factory!(HttpUpdateReceiver)
        }
    }
}
