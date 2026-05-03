use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

#[skip_serializing_none]
#[derive(Debug, Deserialize, Serialize)]
pub struct User {
    pub id: u64,
    pub is_bot: bool,
    pub first_name: String,
    pub last_name: Option<String>,
    pub username: Option<String>,
    #[serde(default)]
    pub is_premium: bool,
    #[serde(default)]
    pub added_to_attachment_menu: bool,
    pub can_join_groups: Option<bool>,
    pub can_read_all_group_messages: Option<bool>,
    pub supports_guest_queries: Option<bool>,
    pub supports_inline_queries: Option<bool>,
    pub can_connect_to_business: Option<bool>,
    pub has_main_web_app: Option<bool>,
    pub has_topics_enabled: Option<bool>,
    pub allows_users_to_create_topics: Option<bool>,
    pub can_manage_bots: Option<bool>
}
