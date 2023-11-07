use dotenvy::dotenv;
use log::*;
use std::env;
use teloxide::prelude::*;
use teloxide::types::{Forward, ForwardedFrom, Message, ParseMode};
use teloxide::types::{MessageCommon, MessageKind};
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
            // let chat_id = message.chat.id;
            let Message { chat, kind, .. } = message.clone();
            if let MessageKind::Common(MessageCommon { forward, .. }) = kind {
                if let Some(Forward { from, .. }) = forward {
                    info!("ForwardedFrom: {:?}", from);
                    match from {
                        ForwardedFrom::User(user) => {
                            let text = format!("GetID: `{}`", user.id);
                            bot.send_message(chat.id, text)
                                .parse_mode(ParseMode::MarkdownV2)
                                .await?;
                        }
                        ForwardedFrom::Chat(chat) => {
                            let text = format!("GetID: `{}`", chat.id);
                            bot.send_message(chat.id, text)
                                .parse_mode(ParseMode::MarkdownV2)
                                .await?;
                        }
                        _ => {}
                    }
                } else {
                    if let Some(text) = message.text() {
                        match text {
                            "id" => {
                                let text = format!("GetID: `{}`", chat.id);
                                bot.send_message(chat.id, text)
                                    .parse_mode(ParseMode::MarkdownV2)
                                    .await?;
                            }
                            _ => {}
                        }
                    }
                }
            }
            Ok(())
        })
        .await;
    }
}
