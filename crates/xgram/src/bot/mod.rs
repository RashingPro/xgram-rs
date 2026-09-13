//! Main struct and managing logic.

pub mod config;
pub mod error;

use crate::bot::config::XgramConfig;
use crate::bot::error::{BotError, BotResult};
use crate::command::context::CommandContext;
use crate::command::{CommandHandler, CommandHandlerFuture};
use colored::Colorize;
use hashbrown::HashMap;
use log::{error, info, trace};
use std::sync::Arc;
use str_indices::utf16;
use xgram_telegram_api::client::TelegramApiClient;
use xgram_telegram_api::types::message::entity::MessageEntityType;
use xgram_telegram_api::types::update::{Update, UpdateKind};
use xgram_telegram_api::update_receiver::UpdateReceiver;
use xgram_utils::types::TokenArc;

/// Main framework's struct.
/// # Example
/// ```rust,ignore
/// use xgram::prelude::*;
///
/// #[tokio::main]
/// async fn main() -> BotResult {
///     dotenv::dotenv().unwrap();
///
///     let token = std::env::var("TOKEN").unwrap();
///
///     let mut bot = Bot::new(token, Default::default());
///     bot.run().await
/// }
/// ```
pub struct Bot {
    client: TelegramApiClient,
    update_receiver: Box<dyn UpdateReceiver>,
    commands: HashMap<String, CommandHandler>
}

impl Bot {
    pub fn new(token: impl Into<String>, config: XgramConfig) -> Self {
        let token = token.into();

        let token: TokenArc = Arc::from(token);
        let XgramConfig {
            bot_config,
            update_receiver
        } = config;

        let bot_config = Arc::new(bot_config);

        let client = TelegramApiClient::new(bot_config.clone(), token.clone());

        Self {
            client: client.clone(),
            update_receiver: update_receiver(bot_config.clone(), client.clone()),
            commands: HashMap::new()
        }
    }

    /// Registers a new bot's command.
    /// # Example
    /// ```rust,ignore
    /// use xgram::prelude::*;
    ///
    /// #[tokio::main]
    /// async fn main() -> BotResult {
    ///     dotenv::dotenv().unwrap();
    ///
    ///     let token = std::env::var("TOKEN").unwrap();
    ///
    ///     let mut bot = Bot::new(token, Default::default());
    ///     bot.register_command("start", command_start);
    ///     bot.run().await
    /// }
    ///
    /// #[command_handler]
    /// async fn command_start(ctx: CommandContext) -> CommandHandlerResult {
    ///     ctx.reply("Hello XGram.rs World!").await?;
    ///     Ok(())
    /// }
    /// ```
    pub fn register_command(
        &mut self,
        trigger: impl Into<String>,
        handler: fn(CommandContext) -> CommandHandlerFuture
    ) {
        let trigger = trigger.into();

        if self.commands.contains_key(&trigger) {
            panic!(
                "command {} already registered",
                format!("/{}", trigger).yellow()
            );
        }
        self.commands.insert(trigger, Box::new(handler));
    }

    /// Consumes the `Bot` instance and runs the main loop.
    ///
    /// Please note, that this function returns `!` (never type) wrapped in
    /// `Result`, which may require a nightly toolchain or an explicit
    /// `#![feature(never_type)]`
    ///
    /// See: https://doc.rust-lang.org/std/primitive.never.html
    pub async fn run(self) -> BotResult {
        info!("Running update polling");

        let mut update_receiver = self.update_receiver.spawn();

        let commands = Arc::new(self.commands);

        while let Some(update) = update_receiver.recv().await {
            match update {
                Ok(update) => {
                    trace!(target: "main_loop", "Received update: {:#?}", update);
                    {
                        let client = self.client.clone();
                        let commands = commands.clone();
                        tokio::spawn(async move {
                            match Self::handle_update(client, update, commands).await {
                                Ok(_) => {
                                    trace!(target: "main_loop", "Successfully handled update");
                                }
                                Err(err) => {
                                    error!(target: "main_loop", "Failed to handle update: {}", err);
                                }
                            }
                        });
                    }
                }
                Err(err) => error!(target: "main_loop", "{}", err)
            }
        }

        Err(BotError::UnreachableCodeReached)
    }

    #[allow(clippy::single_match, reason = "temporary")] // TODO: remove when more update handlers are present
    async fn handle_update(
        client: TelegramApiClient,
        update: Update,
        commands: Arc<HashMap<String, CommandHandler>>
    ) -> Result<(), BotError> {
        match update.update_kind {
            UpdateKind::NewMessage(message) => {
                if let Some(message_text) = &message.text
                    && !message.entities.is_empty()
                {
                    for entity in &message.entities {
                        if let MessageEntityType::BotCommand = entity.entity_type {
                            let start = utf16::to_byte_idx(message_text, entity.offset as usize);
                            let end = utf16::to_byte_idx(
                                message_text,
                                (entity.offset + entity.length) as usize
                            );

                            let command_text = message_text
                                .get(start..end)
                                .and_then(|s| s.strip_prefix('/'))
                                .expect(
                                    "Message text slicing error. This is a bug - please report!"
                                );
                            if let Some(command_handler) = commands.get(command_text) {
                                trace!(
                                    "Handling command {}",
                                    format!("/{}", command_text).yellow()
                                );
                                command_handler(CommandContext::new(client.clone(), message))
                                    .await?;
                                break;
                            }
                        }
                    }
                }
            }
            _ => {}
        }

        Ok(())
    }
}
