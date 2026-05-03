use crate::endpoints::{TelegramApiEndpoint, craft_default_url};
use crate::types::link_preview_options::LinkPreviewOptions;
use crate::types::message::Message;
use crate::types::message::entity::MessageEntity;
use crate::types::message::reply_parameters::ReplyParameters;
use crate::types::parse_mode::ParseMode;
use serde::Serialize;
use serde_with::skip_serializing_none;
use smallvec::SmallVec;

#[skip_serializing_none]
#[derive(Serialize, Default, Debug)]
pub struct SendMessageEndpoint {
    pub chat_id: i64,
    pub text: String,
    pub business_connection_id: Option<String>,
    pub message_thread_id: Option<u32>,
    pub direct_messages_topic_id: Option<u32>,
    pub parse_mode: Option<ParseMode>,
    pub entities: Option<SmallVec<[MessageEntity; 4]>>,
    pub link_preview_options: Option<LinkPreviewOptions>,
    pub disable_notification: Option<bool>,
    pub protect_content: Option<bool>,
    /// May spend Telegram Stars from bot's balance. Use with caution.
    pub allow_paid_broadcast: Option<bool>,
    pub message_effect_id: Option<String>,
    pub suggested_post_parameters: Option<()>, // TODO
    pub reply_parameters: Option<ReplyParameters>,
    pub reply_markup: Option<()> // TODO
}

impl TelegramApiEndpoint<Message> for SendMessageEndpoint {
    fn craft_url(&self, token: &str, base_url: &str) -> String {
        craft_default_url(token, base_url, "sendMessage")
    }
}
