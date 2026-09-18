use xgram_telegram_api::client::TelegramApiClient;
use xgram_telegram_api::endpoints::send_message::SendMessageEndpoint;
use xgram_telegram_api::error::TelegramApiError;
use xgram_telegram_api::types::message::Message;
use xgram_telegram_api::types::message::reply_parameters::ReplyParameters;

pub struct CommandContext {
    message: Message,
    client: TelegramApiClient
}

impl CommandContext {
    pub fn new(client: TelegramApiClient, message: Message) -> Self {
        Self { client, message }
    }

    pub async fn reply(&self, text: impl Into<String>) -> Result<Message, TelegramApiError> {
        let text = text.into();

        self.client
            .make_request(&SendMessageEndpoint {
                chat_id: self.message.chat.id,
                text,
                reply_parameters: Some(ReplyParameters {
                    message_id: self.message.message_id
                }),
                ..Default::default()
            })
            .await
    }
}
