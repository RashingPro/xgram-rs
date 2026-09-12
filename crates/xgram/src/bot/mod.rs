use crate::command::context::CommandContext;
use crate::command::{CommandHandler, CommandHandlerFuture};
use colored::Colorize;
use hashbrown::HashMap;
use log::{error, trace};
use std::sync::Arc;
use xgram_telegram_api::client::TelegramApiClient;
use xgram_telegram_api::types::message::entity::MessageEntityType;
use xgram_telegram_api::types::update::{Update, UpdateKind};
use xgram_telegram_api::update_receiver::UpdateReceiver;
use xgram_telegram_api::update_receiver::http::HttpUpdateReceiver;
use xgram_utils::config::BotConfig;
use xgram_utils::types::{BotConfigArc, TokenArc};

pub struct Bot {
    config: BotConfigArc,
    client: TelegramApiClient,
    commands: HashMap<String, CommandHandler>
}

impl Bot {
    pub fn new(token: String, config: BotConfig) -> Self {
        let token: TokenArc = Arc::from(token);
        let config = Arc::new(config);

        Self {
            config: config.clone(),
            client: TelegramApiClient::new(config.clone(), token.clone()),
            commands: HashMap::new()
        }
    }

    pub fn register_command(
        &mut self,
        trigger: String,
        handler: fn(CommandContext) -> CommandHandlerFuture
    ) {
        if self.commands.contains_key(&trigger) {
            panic!(
                "command {} already registered",
                format!("/{}", trigger).yellow()
            );
        }
        self.commands.insert(trigger, Box::new(handler));
    }

    pub async fn run(self) {
        let update_receiver = HttpUpdateReceiver::new(self.config.clone(), self.client.clone());
        let mut update_receiver = update_receiver.spawn();

        let slf = Arc::new(self);
        while let Some(update) = update_receiver.recv().await {
            match update {
                Ok(update) => {
                    trace!(target: "main_loop", "Received update: {:#?}", update);
                    {
                        let slf = slf.clone();
                        tokio::spawn(async move { slf.handle_update(update).await });
                    }
                }
                Err(err) => error!(target: "main_loop", "{}", err)
            }
        }
    }

    #[allow(clippy::single_match)] // TODO: remove when more update handlers are present
    async fn handle_update(&self, update: Update) {
        match update.update_kind {
            UpdateKind::NewMessage(message) => {
                if let Some(message_text) = &message.text
                    && let Some(entities) = &message.entities
                {
                    for entity in entities {
                        if let MessageEntityType::BotCommand = entity.entity_type {
                            let command_text = &message_text[entity.offset as usize + 1
                                ..(entity.offset + entity.length) as usize];
                            if let Some(command_handler) = self.commands.get(command_text) {
                                trace!(
                                    "Handling command {}",
                                    format!("/{}", command_text).yellow()
                                );
                                command_handler(CommandContext::new(self.client.clone(), message))
                                    .await;
                                break;
                            }
                        }
                    }
                }
            }
            _ => {}
        }
    }
}
