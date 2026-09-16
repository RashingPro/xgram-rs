/// This is probably _not_ what you are looking for. See
/// `xgram::bot::config::BotConfig`
#[derive(Debug)]
pub struct InnerConfig {
    pub api_base_url: &'static str,
    pub update_buffer_capacity: usize,
    pub http_updates_polling_timeout: u8
}
