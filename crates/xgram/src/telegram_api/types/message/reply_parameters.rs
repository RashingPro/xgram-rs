use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct ReplyParameters {
    pub message_id: u32 // TODO
}
