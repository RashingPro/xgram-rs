use std::sync::OnceLock;

static GLOBAL_CONFIG: OnceLock<GlobalConfig> = OnceLock::new();

#[derive(Debug)]
pub struct GlobalConfig {
    pub base_api_url: &'static str,
    pub updates_channel_capacity: usize,
    pub updates_http_polling_timeout: u8
}

impl Default for GlobalConfig {
    fn default() -> Self {
        Self {
            base_api_url: "https://api.telegram.org/",
            updates_channel_capacity: 32,
            updates_http_polling_timeout: 10
        }
    }
}

pub fn init_config(config: GlobalConfig) {
    GLOBAL_CONFIG.set(config).unwrap();
}

pub fn get_config() -> &'static GlobalConfig {
    GLOBAL_CONFIG
        .get()
        .unwrap_or_else(|| panic!("global config was not initialized"))
}
