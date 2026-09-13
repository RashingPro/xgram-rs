#[derive(Debug)]
pub struct BotConfig {
    /// Base URL for Telegram API. Must include trailing slash. Defaults to `https://api.telegram.org/`.
    pub api_base_url: &'static str,
    /// Capacity of MPSC channel for `UpdateReceiver`, which acts like an
    /// update buffer. Higher values requires more memory in runtime.
    /// Increase if your bot is under high load. Defaults to 32.
    pub updates_channel_capacity: usize,
    /// Timeout for HTTP long-polling. Only affects `HttpUpdateReceiver`.
    /// Defaults to 30.
    pub http_updates_polling_timeout: u8
}

impl Default for BotConfig {
    fn default() -> Self {
        Self {
            api_base_url: "https://api.telegram.org/",
            updates_channel_capacity: 32,
            http_updates_polling_timeout: 30
        }
    }
}
