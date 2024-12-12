use crate::configuration::{get_telegram_bot_configuration};
use teloxide::Bot;
use teloxide::prelude::{ChatId, Message};
use teloxide::requests::Requester;
use teloxide::types::MessageId;

pub struct TelegramBot {
    bot: Bot,
    chat_id: String,
}

impl TelegramBot {
    pub async fn try_new() -> TelegramBot {
        let bot_configuration = get_telegram_bot_configuration().await;
        let bot = Bot::new(bot_configuration.token);
        TelegramBot { bot , chat_id: bot_configuration.chat_id}
    }
    
    pub async fn send_message<T>(&self, message: T) -> Message 
        where T: Into<String> {
        self.bot.send_message(self.chat_id.clone(), message).await.expect("Should send message")
    }
    
    pub async fn send_location(&self, latitude: f64, longitude: f64) -> Message {
        self.bot.send_location(self.chat_id.clone(), latitude, longitude).await.expect("Should send location")
    }
    
    pub async fn delete_message(&self, chat_id: i64, message_id_info: i32, message_id_location: i32) {
        self.bot.delete_messages(ChatId(chat_id), vec![MessageId(message_id_info), MessageId(message_id_location)]).await.expect("Should delete message");
    }
}

