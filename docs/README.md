<div align="center">
<h1>XGram.rs</h1>

<img alt="Crates.io Version" src="https://img.shields.io/crates/v/xgram">
<img alt="Crates.io Total Downloads" src="https://img.shields.io/crates/d/xgram">
<img alt="Telegram API Version Badge" src="https://img.shields.io/badge/Telegram%20API%20version-10.0-44bb00">
<img alt="Deps.rs Crate Dependencies (latest)" src="https://img.shields.io/deps-rs/xgram/latest">
<img alt="GitHub License" src="https://img.shields.io/github/license/RashingPro/xgram-rs">

Powerful yet blazingly fast Telegram Bot API framework written in Rust, featuring advanced abstractions and
Tokio-powered concurrency.
</div>

## Using

### Creating bot

Open chat with [@BotFather](https://t.me/BotFather) in Telegram. Create a bot with `/newbot` command. In the end, you
will receive token - a string like this `1234567890:ABCdefGH-AbCd318_someRandom`.  
Don't share the token with anybody (token is like a password to control bot), don't store it directly in code and don't
commit to VCS. Instead, create `.env` file and put it there:

```dotenv
TOKEN=1234567890:ABCdefGH-AbCd318_someRandom
```

### Adding dependencies

```toml
[dependencies]
xgram = "..."
dotenv = "..."
pretty_env_logger = "..." # You can use whatever logger you like
tokio = { version = "...", features = ["rt-multi-thread", "macros"] }
```

### Hello World Bot

```rust
use xgram::prelude::*;

#[tokio::main]
async fn main() {
    dotenv::dotenv().unwrap();
    pretty_env_logger::init();

    let token = std::env::var("TOKEN").unwrap();
    let config = BotConfig {
        ..Default::default()
    };

    let mut bot = Bot::new(token, config);
    bot.register_command(String::from("start"), command_start);
    bot.run().await;
}

#[command_handler]
async fn command_start(ctx: CommandContext) {
    ctx.reply(String::from("Hello XGram.rs World!")).await.unwrap();
}
```

Run your bot and try sending `/start` command.

Complete documentation available here: _\*crickets sound\*_
