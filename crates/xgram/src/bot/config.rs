pub struct BotConfig {
    /// Base URL for Telegram API. Must include trailing slash. Defaults to <`https://api.telegram.org/`>.
    pub api_base_url: &'static str,
    /// Capacity of the MPSC channel used by
    /// [`crate::telegram_api::update_receiver::UpdateReceiver`] as a local
    /// update buffer. When the buffer is full, the update receiver waits
    /// until space becomes available, applying backpressure to the update
    /// source.
    ///
    /// This is an advanced configuration option. The optimal capacity
    /// and tuning strategy are application-dependent and may vary depending
    /// on the used [`crate::telegram_api::update_receiver::UpdateReceiver`]
    /// implementation and processing workload. Higher values allow more
    /// updates to be buffered locally, but may increase memory usage when
    /// the buffer contains many updates.
    ///
    /// Defaults to 32.
    ///
    /// See: [`tokio::sync::mpsc::channel`]
    pub update_buffer_capacity: usize,
    /// Timeout for HTTP long-polling. Only affects
    /// [`crate::telegram_api::update_receiver::long_polling::LongPollingUpdateReceiver`].
    /// Defaults to 30.
    pub update_long_polling_timeout: u8
}

impl Default for BotConfig {
    fn default() -> Self {
        Self {
            api_base_url: "https://api.telegram.org/",
            update_buffer_capacity: 32,
            update_long_polling_timeout: 30
        }
    }
}
