use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

#[skip_serializing_none]
#[derive(Debug, Deserialize, Serialize)]
pub struct LinkPreviewOptions {
    pub is_disabled: Option<bool>,
    pub url: Option<String>,
    pub prefer_small_media: Option<bool>,
    pub prefer_large_media: Option<bool>,
    pub show_above_text: Option<bool>
}
