//! Main struct and managing logic.

pub mod command;
pub mod error;

use crate::{
    bot::{
        command::{CommandHandler, CommandHandlerFuture, context::CommandContext},
        error::{BotError, BotResult}
    },
    telegram_api::{
        client::TelegramApiClient,
        types::{
            message::entity::MessageEntityType,
            update::{Update, UpdateKind}
        },
        update_receiver::{UpdateReceiver, long_polling::LongPollingUpdateReceiver}
    },
    utils::{
        config::Config,
        types::{ConfigArc, TokenArc}
    }
};
use colored::Colorize;
use hashbrown::HashMap;
use log::{error, info, trace, warn};
use std::{marker::PhantomData, sync::Arc};
use str_indices::utf16;
use tokio::task::JoinHandle;

/// Main framework's struct.
/// # Example
/// ```rust,ignore
/// use xgram::prelude::*;
///
/// #[tokio::main]
/// async fn main() {
///     dotenvy::dotenv().expect("failed to load .env");
///     pretty_env_logger::init();
///
///     let token = std::env::var("TOKEN").unwrap();
///
///     let mut bot = Bot::new(token, Default::default());
///     bot.run().await.unwrap();
/// }
/// ```
pub struct Bot<U = LongPollingUpdateReceiver>
where
    U: UpdateReceiver
{
    client: TelegramApiClient,
    commands: HashMap<String, CommandHandler>,
    config: ConfigArc,
    _u: PhantomData<U>
}

impl<U> Bot<U>
where
    U: UpdateReceiver
{
    pub fn new(token: impl Into<String>, config: Config) -> Self {
        let token = token.into();

        let token: TokenArc = Arc::from(token);
        let config: ConfigArc = Arc::new(config);

        let client = TelegramApiClient::new(config.clone(), token.clone());

        Self {
            client: client.clone(),
            commands: HashMap::new(),
            config,
            _u: Default::default()
        }
    }

    /// Registers a new bot's command.
    /// # Example
    /// ```rust,ignore
    /// use xgram::prelude::*;
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///     dotenvy::dotenv().expect("failed to load .env");
    ///
    ///     let token = std::env::var("TOKEN").expect("TOKEN environment variable is not set");
    ///
    ///     let mut bot: Bot = Bot::new(token, Default::default());
    ///     bot.register_command("start", command_start);
    ///     bot.run().await.unwrap();
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
        self.commands.insert(trigger, handler);
    }

    /// Consumes the `Bot` instance and runs the main loop.
    ///
    /// Please note, that this function returns `!` (never type) wrapped in
    /// [`Result`], which currently requires a nightly toolchain.
    ///
    /// See: [`never`]
    pub async fn run(self) -> BotResult {
        info!(target: "xgram::main_loop", "Running update polling");

        let mut update_receiver = U::new(self.config.clone(), self.client.clone()).spawn();
        let commands = Arc::new(self.commands);
        drop(self.config);

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

    pub fn run_detach(self) -> JoinHandle<BotResult> {
        tokio::spawn(self.run())
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
                        // TODO: /foo@bar syntax parsing
                        // TODO: more flexible API for !non_standard_command_syntax
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
