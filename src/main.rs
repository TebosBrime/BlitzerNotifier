use crate::telegram::TelegramBot;

mod blitzer_api_client;
mod database;
mod model;
mod configuration;
mod telegram;
mod handler;

#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {
    let telegram_bot = TelegramBot::try_new().await;

    if let Err(error) = handler::handle(&telegram_bot).await {
        eprintln!("Error: {}", error);
        telegram_bot.send_message(format!("Failed to execute bot... see log for more information. {}", error)).await;
    }
    
    Ok(())
}

