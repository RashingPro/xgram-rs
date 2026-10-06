#![cfg(test)]

mod utils;

use crate::utils::{before_test, get_token};
use serde_json::json;
use std::time::Duration;
use wiremock::{
    Mock,
    MockServer,
    ResponseTemplate,
    matchers::{method, path}
};
use xgram::{
    bot::{Bot, command::CommandHandlerFuture},
    prelude::{CommandContext, Config},
    telegram_api::{
        endpoints::send_message::SendMessageEndpoint,
        update_receiver::long_polling::LongPollingUpdateReceiver
    }
};

#[tokio::test]
async fn commands_1() {
    before_test();
    let token = get_token();

    let mock_server = MockServer::start().await;

    const MESSAGE_ID: u32 = 123;
    const CHAT_ID: i64 = 123456;
    const REPLY_MESSAGE_TEXT: &str = "Hello, World!";

    Mock::given(method("POST"))
        .and(path(format!("/bot{}/getUpdates", token)))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!(
            {
                "ok": true,
                "result": [
                    {
                        "update_id": 1,
                        "message": {
                            "message_id": MESSAGE_ID,
                            "date": 456,
                            "chat": {
                                "id": CHAT_ID,
                                "type": "private"
                            },
                            "text": "/start",
                            "entities": [
                                {
                                    "type": "bot_command",
                                    "offset": 0,
                                    "length": 6
                                }
                            ]
                        }
                    }
                ]
            }
        )))
        .up_to_n_times(1)
        .mount(&mock_server)
        .await;

    Mock::given(method("POST"))
        .and(path(format!("/bot{}/getUpdates", token)))
        .respond_with(
            ResponseTemplate::new(200)
                .set_body_json(json!(
                    {
                        "ok": true,
                        "result": []
                    }
                ))
                .set_delay(Duration::from_secs(30))
        )
        .mount(&mock_server)
        .await;

    let guard = Mock::given(method("POST"))
        .and(path(format!("/bot{}/sendMessage", token)))
        .respond_with(ResponseTemplate::new(200))
        .expect(1)
        .mount_as_scoped(&mock_server)
        .await;

    let mut bot = Bot::<LongPollingUpdateReceiver>::new(
        token.clone(),
        Config {
            api_base_url: mock_server.uri() + "/",
            ..Default::default()
        }
    );
    bot.register_command("start", |ctx: CommandContext| -> CommandHandlerFuture {
        Box::pin(async move {
            ctx.reply(REPLY_MESSAGE_TEXT).await?;
            Ok(())
        })
    });

    let bot = bot.run_detach();

    println!("Waiting");

    guard.wait_until_satisfied().await;

    bot.abort();
    println!("Aborted");
    bot.await.unwrap_err();

    println!(
        "{:#?}",
        mock_server
            .received_requests()
            .await
            .unwrap_or_default()
            .iter()
            .map(|r| format!("{}", r.url))
            .collect::<Vec<String>>()
    );

    let mut is_ok = false;
    for req in mock_server.received_requests().await.unwrap_or_default() {
        if req.url.path() != format!("/bot{}/sendMessage", token) {
            continue;
        }

        let body: SendMessageEndpoint = req.body_json().unwrap();
        assert_eq!(body.chat_id, CHAT_ID);
        assert_eq!(body.text, REPLY_MESSAGE_TEXT);
        assert_eq!(body.reply_parameters.unwrap().message_id, MESSAGE_ID);
        is_ok = true;
    }
    assert!(is_ok);
}
