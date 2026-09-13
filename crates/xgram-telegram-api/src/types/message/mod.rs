use crate::types::chat::Chat;
use crate::types::link_preview_options::LinkPreviewOptions;
use crate::types::message::entity::MessageEntity;
use crate::types::user::User;
use serde::{Deserialize, Deserializer};
use smallvec::SmallVec;

pub mod entity;
pub mod reply_parameters;

#[derive(Deserialize, Debug)]
pub struct Message {
    pub message_id: u32,
    pub message_thread_id: Option<u32>,
    pub direct_messages_topic: Option<()>, // TODO

    pub from: Option<User>,
    pub sender_chat: Option<Chat>,
    pub sender_boost_count: Option<u16>,
    pub sender_business_bot: Option<User>,
    pub sender_tag: Option<String>,

    pub receiver_user: Option<User>,
    pub ephemeral_message_id: Option<u32>,
    pub date: u32,
    pub guest_query_id: Option<String>,
    pub business_connection_id: Option<String>,
    pub chat: Chat,
    pub forward_origin: Option<()>, // TODO
    #[serde(default)]
    pub is_topic_message: bool,
    #[serde(default)]
    pub is_automatic_forward: bool,

    #[serde(default, deserialize_with = "deserialize_reply_to_message")]
    pub reply_to_message: ReplyToMessage,
    pub external_reply: Option<()>, // TODO
    pub quote: Option<()>,          // TODO
    pub reply_to_story: Option<()>, // TODO
    pub reply_to_checklist_task_id: Option<u32>,
    pub reply_to_poll_option_id: Option<String>,

    pub via_bot: Option<User>,

    pub guest_bot_caller_user: Option<User>,
    pub guest_bot_caller_chat: Option<Chat>,

    pub edit_date: Option<u32>,
    #[serde(default)]
    pub has_protected_content: bool,
    #[serde(default)]
    pub is_from_offline: bool,
    #[serde(default)]
    pub is_paid_post: bool,
    pub media_group_id: Option<String>,
    pub author_signature: Option<String>,
    pub paid_star_count: Option<u16>,
    pub text: Option<String>,
    #[serde(default)]
    pub entities: SmallVec<[MessageEntity; 4]>,
    pub link_preview_options: Option<LinkPreviewOptions>,
    pub suggested_post_info: Option<()>, // TODO
    pub effect_id: Option<String>,

    pub rich_message: Option<()>, // TODO
    pub animation: Option<()>,    // TODO
    pub audio: Option<()>,        // TODO
    pub document: Option<()>,     // TODO
    pub live_photo: Option<()>,   // TODO
    pub paid_media: Option<()>,   // TODO
    #[serde(default)]
    pub photo: Vec<()>, // TODO
    pub sticker: Option<()>,      // TODO
    pub story: Option<()>,        // TODO
    pub video: Option<()>,        // TODO
    pub video_note: Option<()>,   // TODO
    pub voice: Option<()>,        // TODO

    pub caption: Option<String>, // TODO
    #[serde(default)]
    pub caption_entities: SmallVec<[MessageEntity; 4]>,
    #[serde(default)]
    pub show_caption_above_media: bool,
    #[serde(default)]
    pub has_media_spoiler: bool,

    pub checklist: Option<()>, // TODO
    pub contact: Option<()>,   // TODO
    pub dice: Option<()>,      // TODO
    pub game: Option<()>,      // TODO
    pub poll: Option<()>,      // TODO
    pub venue: Option<()>,     // TODO
    pub location: Option<()>,  // TODO

    #[serde(default)]
    pub new_chat_members: SmallVec<[User; 4]>, // TODO
    pub left_chat_member: Option<User>, // TODO
    pub chat_owner_left: Option<()>,    // TODO
    pub chat_owner_changed: Option<()>, // TODO

    pub new_chat_title: Option<String>, // TODO
    #[serde(default)]
    pub new_chat_photo: Vec<()>, // TODO
    #[serde(default)]
    pub delete_chat_photo: bool,
    #[serde(default)]
    pub group_chat_created: bool,
    #[serde(default)]
    pub supergroup_chat_created: bool,
    #[serde(default)]
    pub channel_chat_created: bool,
    pub message_auto_delete_timer_changed: Option<()>, // TODO
    pub migrate_to_chat_id: Option<u64>,
    pub migrate_from_chat_id: Option<u64>,

    pub pinned_message: Option<()>, // TODO

    pub invoice: Option<()>,            // TODO
    pub successful_payment: Option<()>, // TODO
    pub refunded_payment: Option<()>,   // TODO

    pub users_shared: Option<()>, // TODO
    pub chat_shared: Option<()>,  // TODO

    pub gift: Option<()>,              // TODO
    pub unique_gift: Option<()>,       // TODO
    pub gift_upgrade_sent: Option<()>, // TODO

    pub connected_website: Option<String>,     // TODO
    pub write_access_allowed: Option<()>,      // TODO
    pub passport_data: Option<()>,             // TODO
    pub proximity_alert_triggered: Option<()>, // TODO
    pub boost_added: Option<()>,               // TODO
    pub chat_background_set: Option<()>,       // TODO

    pub checklist_tasks_done: Option<()>,  // TODO
    pub checklist_tasks_added: Option<()>, // TODO

    pub community_chat_added: Option<()>,   // TODO
    pub community_chat_joined: Option<()>,  // TODO
    pub community_chat_removed: Option<()>, // TODO

    pub direct_message_price_changed: Option<()>, // TODO

    pub forum_topic_created: Option<()>,          // TODO
    pub forum_topic_edited: Option<()>,           // TODO
    pub forum_topic_closed: Option<()>,           // TODO
    pub forum_topic_reopened: Option<()>,         // TODO
    pub general_forum_topic_hidden: Option<()>,   // TODO
    pub general_forum_topic_unhidden: Option<()>, // TODO

    pub giveaway_created: Option<()>,   // TODO
    pub giveaway: Option<()>,           // TODO
    pub giveaway_winners: Option<()>,   // TODO
    pub giveaway_completed: Option<()>, // TODO

    pub managed_bot_created: Option<()>,        // TODO
    pub paid_message_price_changed: Option<()>, // TODO

    pub poll_option_added: Option<()>,   // TODO
    pub poll_option_deleted: Option<()>, // TODO

    pub suggested_post_approved: Option<()>,        // TODO
    pub suggested_post_approval_failed: Option<()>, // TODO
    pub suggested_post_declined: Option<()>,        // TODO
    pub suggested_post_paid: Option<()>,            // TODO
    pub suggested_post_refunded: Option<()>,        // TODO

    pub video_chat_scheduled: Option<()>,            // TODO
    pub video_chat_started: Option<()>,              // TODO
    pub video_chat_ended: Option<()>,                // TODO
    pub video_chat_participants_invited: Option<()>, // TODO

    pub web_app_data: Option<()>, // TODO
    pub reply_markup: Option<()>  // TODO
}

/// Telegram Bot API truncates nested `reply_to_message` fields.
/// As a result, inner messages may not contain reply information even if _they
/// are_ replies themselves.
///
/// `Truncated` means that reply relationship is unknown due to Telegram API
/// limitations. Please note, that this state can _not_ be validated as Telegram
/// Bot API does _not_ provide a method for fetching messages by their IDs.
///
/// `Complete` means that reply relationship is known exactly:
/// - `Complete(Some(_))` — message is a reply
/// - `Complete(None)` — message is definitely not a reply
///
/// See: https://core.telegram.org/bots/api#message
#[derive(Debug)]
pub enum ReplyToMessage {
    Truncated,
    Complete(Option<Box<Message>>)
}

impl Default for ReplyToMessage {
    fn default() -> Self {
        Self::Complete(None)
    }
}

fn deserialize_reply_to_message<'de, D>(deserializer: D) -> Result<ReplyToMessage, D::Error>
where
    D: Deserializer<'de>
{
    let mut map = serde_json::Map::deserialize(deserializer)?;

    let reply_to_message = map.remove("reply_to_message");

    let inner_reply_to_message = match reply_to_message {
        None => ReplyToMessage::Truncated,
        Some(v) => ReplyToMessage::Complete(Some(Box::new(
            serde_json::from_value(v).map_err(serde::de::Error::custom)?
        )))
    };

    let mut message = serde_json::from_value::<Message>(serde_json::Value::Object(map))
        .map_err(serde::de::Error::custom)?; // Deserialization isn't failing because of Default trait implemented for ReplyToMessage
    message.reply_to_message = inner_reply_to_message;

    Ok(ReplyToMessage::Complete(Some(Box::new(message))))
}
