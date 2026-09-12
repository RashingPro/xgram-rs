#[derive(Debug)]
pub struct BotConfig {
    pub api_base_url: &'static str,
    pub updates_channel_capacity: usize,
    pub updates_http_polling_timeout: u8
}

impl Default for BotConfig {
    fn default() -> Self {
        Self {
            api_base_url: "https://api.telegram.org/",
            updates_channel_capacity: 32,
            updates_http_polling_timeout: 30
        }
    }
}
