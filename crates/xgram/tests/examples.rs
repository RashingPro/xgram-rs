#![cfg(test)]

mod utils;

mod readme {
    use crate::utils::before_test;
    use std::{assert_matches, time::Duration};
    use xgram::{bot::error::BotResult, prelude::*};

    #[tokio::test]
    async fn test() {
        let result = tokio::time::timeout(Duration::from_secs(5), main()).await; // Expect program to run for 5 seconds
        assert_matches!(result, Err(..)) // Error means program did run for 5 seconds, which is what we are expecting
    }

    async fn main() -> BotResult {
        before_test();

        let token = std::env::var("TOKEN").expect("TOKEN environment variable is not set");
        let config = Config {
            ..Default::default()
        };

        let mut bot: Bot = Bot::new(token, config);
        bot.register_command("start", command_start);
        bot.run().await
    }

    #[command_handler]
    async fn command_start(ctx: CommandContext) -> CommandHandlerResult {
        ctx.reply("Hello XGram.rs World!").await?;
        Ok(())
    }
}
