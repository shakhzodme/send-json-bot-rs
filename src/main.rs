use std::io::Error;

use teloxide::{
    dispatching::{dialogue::GetChatId, UpdateHandler},
    prelude::*,
    types::ParseMode,
};

fn prettify_message<T>(msg: &T) -> String
where
    T: ?Sized + serde::Serialize,
{
    let data = serde_json::to_string_pretty(msg);
    let data = match data {
        Ok(data) => format!("```json\n{}\n```", data),
        Err(_) => "Failed to recognize".to_string(),
    };
    data
}

#[tokio::main]
async fn main() {
    dotenvy::dotenv().ok();
    pretty_env_logger::init();

    let bot = Bot::from_env();
    let handler: UpdateHandler<Error> =
        dptree::entry().endpoint(|u: Update, bot: Bot| async move {
            if let None = u.chat_id() {
                return Ok(());
            }

            let _ = bot
                .send_message(u.chat_id().unwrap(), prettify_message(&u))
                .parse_mode(ParseMode::MarkdownV2)
                .await;

            Ok(())
        });

    teloxide::dispatching::Dispatcher::builder(bot.clone(), handler)
        .build()
        .dispatch()
        .await;
}
