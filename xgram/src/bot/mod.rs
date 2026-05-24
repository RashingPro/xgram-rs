use crate::command::CommandHandler;
use crate::command::context::CommandContext;
use colored::Colorize;
use hashbrown::HashMap;
use log::{error, trace};
use std::sync::Arc;
use xgram_telegram_api::client::TelegramApiClient;
use xgram_telegram_api::types::message::entity::MessageEntityType;
use xgram_telegram_api::types::update::UpdateKind;
use xgram_telegram_api::update_receiver::UpdateReceiver;
use xgram_telegram_api::update_receiver::http::HttpUpdateReceiver;
use xgram_utils::config::{GlobalConfig, init_config};

pub struct Bot {
    client: TelegramApiClient,
    commands: HashMap<String, CommandHandler>
}

impl Bot {
    pub fn new(token: String, config: GlobalConfig) -> Self {
        let token = Arc::new(token);

        init_config(config);

        Self {
            client: TelegramApiClient::new(token.clone()),
            commands: HashMap::new()
        }
    }

    pub fn register_command(&mut self, trigger: String, handler: CommandHandler) {
        if self.commands.contains_key(&trigger) {
            panic!("command {} already registered", format!("/{}", trigger).yellow());
        }
        self.commands.insert(trigger, handler);
    }

    pub async fn run(self) {
        let update_receiver = HttpUpdateReceiver::new(self.client.clone());
        let mut update_receiver = update_receiver.spawn();

        while let Some(update) = update_receiver.recv().await {
            match update {
                Ok(update) => {
                    trace!(target: "main_loop","Received update: {:#?}", update);
                    if let UpdateKind::NewMessage(message) = update.update_kind
                        && let Some(message_text) = &message.text
                        && let Some(entities) = &message.entities
                    {
                        for entity in entities {
                            if let MessageEntityType::BotCommand = entity.entity_type {
                                let command_text =
                                    &message_text[entity.offset as usize + 1..(entity.offset + entity.length) as usize];
                                if let Some(command_handler) = self.commands.get(command_text) {
                                    trace!("Handling command {}", format!("/{}", command_text).yellow());
                                    tokio::spawn(command_handler(CommandContext::new(self.client.clone(), message)));
                                    break;
                                }
                            }
                        }
                    }
                }
                Err(err) => error!(target: "main_loop", "{}", err)
            }
        }
    }
}
