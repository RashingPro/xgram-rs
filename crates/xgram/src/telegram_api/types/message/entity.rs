use crate::telegram_api::types::user::User;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct MessageEntity {
    #[serde(flatten)]
    pub entity_type: MessageEntityType,
    pub offset: u16,
    pub length: u16
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(tag = "type", rename_all = "snake_case")]
#[non_exhaustive]
pub enum MessageEntityType {
    Mention,
    Hashtag,
    Cashtag,
    BotCommand,
    Url,
    Email,
    PhoneNumber,
    Bold,
    Italic,
    Underline,
    Strikethrough,
    Spoiler,
    Blockquote,
    ExpandableBlockquote,
    Code,
    Pre {
        language: Option<String>
    },
    TextLink {
        url: String
    },
    TextMention {
        user: User
    },
    CustomEmoji {
        custom_emoji_id: String
    },
    DateTime {
        unix_time: u32,
        date_time_format: String
    }
}
