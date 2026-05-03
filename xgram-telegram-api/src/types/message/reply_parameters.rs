use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct ReplyParameters {
    pub message_id: u32 // TODO
}
