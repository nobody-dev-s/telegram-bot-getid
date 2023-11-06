use dotenvy::dotenv;
use log::*;
use std::env;
use teloxide::prelude::*;
use teloxide::types::Message;
use teloxide::Bot;
use tracing_subscriber;

#[tokio::main]
async fn main() {
    dotenv().expect(".env file not found");
    tracing_subscriber::fmt::init();
    let bot = GetIDBot::new(env::var("token").expect("token not found"));
    bot.start().await;
}

pub struct GetIDBot {
    pub bot: Bot,
}

impl GetIDBot {
    pub fn new<S>(token: S) -> Self
    where
        S: Into<String>,
    {
        let bot = Bot::new(token);
        Self { bot }
    }

    pub async fn start(&self) {
        teloxide::repl(self.bot.clone(), |bot: Bot, message: Message| async move {
            let chat_id = message.chat.id;
            info!("received message text {}", message.text().unwrap_or(""));
            if let Some(text) = message.text() {
                match text {
                    "id" => {
                        let text = format!("GetID: {}", chat_id);
                        bot.send_message(chat_id, text).await?;
                    }
                    _ => {}
                }
            }
            Ok(())
        })
        .await;
    }
}
