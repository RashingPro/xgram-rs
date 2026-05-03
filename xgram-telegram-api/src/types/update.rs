use crate::types::message::Message;
use serde::{Deserialize, Deserializer};
use serde_json::Value;

#[derive(Debug)]
pub struct Update {
    pub update_id: u32,
    pub update_kind: UpdateKind
}

/// See: https://core.telegram.org/bots/api#update
#[derive(Debug)]
pub enum UpdateKind {
    NewMessage(Message),
    EditedMessage(Message),
    NewChannelPost(Message),
    EditedChannelPost(Message),
    BusinessConnection(()), // TODO
    NewBusinessMessage(Message),
    EditedBusinessMessage(Message),
    DeletedBusinessMessage(()), // TODO
    GuestMessage(Message),
    /// Requires `message_reaction` in the list of `allowed_updates`
    MessageReaction(()), // TODO
    /// Requires `message_reaction_count` in the list of `allowed_updates`
    MessageReactionCount(()), // TODO
    InlineQuery(()),         // TODO
    ChosenInlineResult(()),  // TODO
    CallbackQuery(()),       // TODO
    ShippingQuery(()),       // TODO
    PreCheckoutQuery(()),    // TODO
    PurchasedPaidMedia(()),  // TODO
    Poll(()),                // TODO
    PollAnswer(()),          // TODO
    MyChatMemberUpdated(()), // TODO
    /// Requires `chat_member` in the list of `allowed_updates`
    ChatMemberUpdated(()), // TODO
    ChatJoinRequest(()),     // TODO
    ChatBoostUpdated(()),    // TODO
    ChatBoostRemoved(()),    // TODO
    ManagedBotUpdated(())    // TODO
}

impl<'de> Deserialize<'de> for Update {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>
    {
        let mut map = serde_json::Map::deserialize(deserializer)?;

        if map.len() != 2 {
            return Err(serde::de::Error::custom(format!(
                "expected 2 fields, found {}",
                map.len()
            )));
        }
        let update_id = map
            .remove("update_id")
            .ok_or(serde::de::Error::missing_field("update_id"))?
            .as_u64()
            .ok_or(serde::de::Error::custom("expected update_id to be integer"))? as u32;

        let (update_body_key, update_body) = map.into_iter().next().unwrap();
        let update_body = match update_body {
            Value::Object(obj) => obj,
            _ => return Err(serde::de::Error::custom("expected update body to be object"))
        };

        macro_rules! update_body_match {
            (
                $match_value:expr,
                $update_body:ident,
                {
                    $($key:literal => $kind:path),* $(,)?
                }
            ) => {
                match $match_value {
                    $($key => $kind(serde_json::from_value(serde_json::Value::Object($update_body)).unwrap()),)*
                    _ => return Err(serde::de::Error::custom(format!("unknown update type: {}", $match_value)))
                }
            };
        }

        let update_kind = update_body_match!(
            update_body_key.as_str(),
            update_body,
            {
                "message" => UpdateKind::NewMessage,
                "edited_message" => UpdateKind::EditedMessage,
                "channel_post" => UpdateKind::NewChannelPost,
                "edited_channel_post" => UpdateKind::EditedChannelPost,
                "business_connection" => UpdateKind::BusinessConnection,
                "business_message" => UpdateKind::NewBusinessMessage,
                "edited_business_message" => UpdateKind::EditedBusinessMessage,
                "deleted_business_messages" => UpdateKind::DeletedBusinessMessage,
                "guest_message" => UpdateKind::GuestMessage,
                "message_reaction" => UpdateKind::MessageReaction,
                "message_reaction_count" => UpdateKind::MessageReactionCount,
                "inline_query" => UpdateKind::InlineQuery,
                "chosen_inline_result" => UpdateKind::ChosenInlineResult,
                "callback_query" => UpdateKind::CallbackQuery,
                "shipping_query" => UpdateKind::ShippingQuery,
                "pre_checkout_query" => UpdateKind::PreCheckoutQuery,
                "purchased_paid_media" => UpdateKind::PurchasedPaidMedia,
                "poll" => UpdateKind::Poll,
                "poll_answer" => UpdateKind::PollAnswer,
                "my_chat_member" => UpdateKind::MyChatMemberUpdated,
                "chat_member" => UpdateKind::ChatMemberUpdated,
                "chat_join_request" => UpdateKind::ChatJoinRequest,
                "chat_boost" => UpdateKind::ChatBoostUpdated,
                "removed_chat_boost" => UpdateKind::ChatBoostRemoved,
                "managed_bot" => UpdateKind::ManagedBotUpdated
            }
        );

        Ok(Self { update_id, update_kind })
    }
}
