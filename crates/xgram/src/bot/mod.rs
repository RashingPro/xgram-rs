//! Main struct and managing logic.

pub mod config;
pub mod error;

use crate::{
    bot::{
        config::BotConfig,
        error::{BotError, BotResult}
    },
    command::{CommandHandler, CommandHandlerFuture, context::CommandContext}
};
use colored::Colorize;
use hashbrown::HashMap;
use log::{error, info, trace, warn};
use std::{marker::PhantomData, sync::Arc};
use str_indices::utf16;
use xgram_telegram_api::{
    client::TelegramApiClient,
    types::{
        message::entity::MessageEntityType,
        update::{Update, UpdateKind}
    },
    update_receiver::{UpdateReceiver, long_polling::LongPollingUpdateReceiver}
};
use xgram_utils::{
    config::InnerConfig,
    types::{InnerConfigArc, TokenArc}
};

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
pub struct Bot<U = LongPollingUpdateReceiver>
where
    U: UpdateReceiver
{
    client: TelegramApiClient,
    commands: HashMap<String, CommandHandler>,
    inner_config: InnerConfigArc,
    _u: PhantomData<U>
}

impl<U> Bot<U>
where
    U: UpdateReceiver
{
    pub fn new(token: impl Into<String>, config: BotConfig) -> Self {
        let token = token.into();

        let token: TokenArc = Arc::from(token);

        let BotConfig {
            api_base_url,
            update_buffer_capacity,
            update_long_polling_timeout: http_updates_polling_timeout
        } = config;
        let inner_config = InnerConfig {
            api_base_url,
            update_buffer_capacity,
            http_updates_polling_timeout
        };
        let inner_config = Arc::new(inner_config);

        let client = TelegramApiClient::new(inner_config.clone(), token.clone());

        Self {
            client: client.clone(),
            commands: HashMap::new(),
            inner_config,
            _u: Default::default()
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
    /// [`Result`], which currently requires a nightly toolchain.
    ///
    /// See: [`never`]
    pub async fn run(self) -> BotResult {
        info!(target: "xgram::main_loop", "Running update polling");

        let mut update_receiver = U::new(self.inner_config.clone(), self.client.clone()).spawn();
        let commands = Arc::new(self.commands);
        drop(self.inner_config);

        while let Some(update) = update_receiver.recv().await {
            match update {
                Ok(update) => {
                    trace!(target: "xgram::main_loop", "Received update: {:#?}", update);
                    {
                        let client = self.client.clone();
                        let commands = commands.clone();
                        tokio::spawn(async move {
                            match Self::handle_update(client, update, commands).await {
                                Ok(_) => {
                                    trace!(target: "xgram::main_loop", "Successfully handled update");
                                }
                                Err(err) => {
                                    error!(target: "xgram::main_loop", "Failed to handle update: {}", err);
                                }
                            }
                        });
                    }
                }
                Err(err) => error!(target: "xgram::main_loop", "{}", err)
            }
        }

        Err(BotError::UnreachableCodeReached)
    }

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
                                    target: "xgram::main_loop",
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
            UpdateKind::Unknown(key) => {
                warn!(target: "xgram::main_loop", "Unknown update kind: `{key}`. This might be API miscoverage. Please consider reporting.");
            }
            _ => {}
        }

        Ok(())
    }
}
