use crate::types::user::User;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct MessageEntity {
    #[serde(flatten)]
    pub entity_type: MessageEntityType,
    pub offset: u16,
    pub length: u16
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(tag = "type", rename_all = "snake_case")]
#[non_exhaustive]
pub enum MessageEntityType {
    Mention,
    Hashtag,
    Cashtag,
    BotCommand,
    Url {
        url: Option<String>
    },
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
    TextLink,
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
