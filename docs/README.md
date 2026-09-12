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

## Usage

### Creating a bot

Open [@BotFather](https://t.me/BotFather) in Telegram. Use the `/newbot` command to create a bot and follow the prompts.
At the end, you will receive a token – a string like `1234567890:ABCdefGH-AbCd318_someRandom`.  
Don't share the token with anyone (treat the token like a password: anyone who has it can control your bot), don't store
it directly in your code and don't commit to the VCS. Instead, create a `.env` file and put it there:

```dotenv
TOKEN=1234567890:ABCdefGH-AbCd318_someRandom
```

You might also want to configure the log level (see the `pretty_env_logger` documentation):

```dotenv
RUST_LOG=info
```

Don't forget to exclude `.env` from the VCS:

```gitignore
# .gitignore
*.env
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
async fn main() -> BotResult {
    dotenv::dotenv().expect("failed to load .env");
    pretty_env_logger::init();

    let token = std::env::var("TOKEN").expect("TOKEN environment variable is not set");
    let config = BotConfig {
        ..Default::default()
    };

    let mut bot = Bot::new(token, config);
    bot.register_command("start", command_start);
    bot.run().await
}

#[command_handler]
async fn command_start(ctx: CommandContext) -> CommandHandlerResult {
    ctx.reply("Hello XGram.rs World!").await?;
    Ok(())
}
```

Run your bot and try sending the `/start` command.

Complete documentation is available here: _\*crickets sound\*_

## Advanced usage

### Bypassing regional censorship

In some countries and regions, Telegram API endpoints may be blocked by local authorities. Because they are just HTTP
endpoints, you can't use MTProto proxy to bypass the restrictions. As a solution, you can use Cloudflare Workers along
with the `base_api_url` config option:

Cloudflare Worker code example:

```js
export default {
    async fetch(request, env, ctx) {
        const url = new URL(request.url);
        url.host = "api.telegram.org";

        const newRequest = new Request(url.toString(), {
            method: request.method,
            headers: request.headers,
            body: request.body,
            redirect: "manual"
        });

        return fetch(newRequest);
    }
};
```

Bot configuration:

```rust
#[tokio::main]
async fn main() -> BotResult {
    let token = "my-awesome-token";
    let config = BotConfig {
        base_api_url: "https://my-cool-worker.my-badass-username.workers.dev/", // URL must include trailing slash
        ..Default::default()
    };

    let mut bot = Bot::new(token, config);
    bot.run().await
}
```

> [!CAUTION]
> Please remember that without additional configuration, your Cloudflare Worker is publicly accessible to anyone.
