use serde::Serialize;

#[derive(Serialize, Debug)]
pub enum ParseMode {
    MarkdownV2,
    HTML,
    Markdown
}
