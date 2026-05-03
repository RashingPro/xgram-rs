use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct Chat {
    pub id: i64,
    pub r#type: ChatType,
    pub title: Option<String>,
    pub username: Option<String>,
    pub first_name: Option<String>,
    pub last_name: Option<String>,
    #[serde(default)]
    pub is_forum: bool,
    #[serde(default)]
    pub is_direct_messages: bool
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "lowercase")]
pub enum ChatType {
    Private,
    Group,
    Supergroup,
    Channel
}
