use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub enum ParseMode {
    MarkdownV2,
    HTML,
    Markdown
}
