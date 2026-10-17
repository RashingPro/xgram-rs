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

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;
    use std::assert_matches;

    #[test]
    fn message_entity_1() {
        let json = json!({
            "type": "bold",
            "offset": 1,
            "length": 3
        });

        let entity: MessageEntity = serde_json::from_value(json).unwrap();
        assert_matches!(entity.entity_type, MessageEntityType::Bold);
    }

    #[test]
    fn message_entity_2() {
        let json = json!({
            "type": "pre",
            "language": "java",
            "offset": 1,
            "length": 3
        });

        let entity: MessageEntity = serde_json::from_value(json).unwrap();
        assert_matches!(
            entity.entity_type,
            MessageEntityType::Pre { language: Some(..) }
        );
    }

    #[test]
    fn message_entity_3() {
        let json = json!({
            "type": "pre",
            "offset": 1,
            "length": 3
        });

        let entity: MessageEntity = serde_json::from_value(json).unwrap();
        assert_matches!(
            entity.entity_type,
            MessageEntityType::Pre { language: None }
        );
    }

    #[test]
    fn message_entity_4() {
        let json = json!({
            "type": "text_link",
            "offset": 1,
            "length": 3
        });

        serde_json::from_value::<MessageEntity>(json).unwrap_err();
    }
}
