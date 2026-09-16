use xgram_telegram_api::update_receiver::http::HttpUpdateReceiver;
use xgram_telegram_api::update_receiver::{UpdateReceiverFactory, update_receiver_factory};

pub struct BotConfig {
    /// Base URL for Telegram API. Must include trailing slash. Defaults to `https://api.telegram.org/`.
    pub api_base_url: &'static str,
    /// Capacity of MPSC channel for `UpdateReceiver`, which acts like an
    /// update buffer. Higher values requires more memory in runtime.
    /// Increase if your bot is under high load. Defaults to 32.
    pub updates_channel_capacity: usize,
    /// Timeout for HTTP long-polling. Only affects `HttpUpdateReceiver`.
    /// Defaults to 30.
    pub http_updates_polling_timeout: u8,
    /// Allows you to choose `UpdateReceiver`. You can use
    /// `update_receiver_factory!(UpdateReceiverStruct)` macro. Defaults to
    /// `update_receiver_factory!(HttpUpdateReceiver)`.
    pub update_receiver: UpdateReceiverFactory
}

impl Default for BotConfig {
    fn default() -> Self {
        Self {
            api_base_url: "https://api.telegram.org/",
            update_buffer_capacity: 32,
            http_updates_polling_timeout: 30,
            update_receiver: update_receiver_factory!(HttpUpdateReceiver)
        }
    }
}
