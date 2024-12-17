use std::time::Duration;
use tokio::time::sleep;
use crate::telegram::TelegramBot;

mod blitzer_api_client;
mod configuration;
mod database;
mod handler;
mod model;
mod telegram;

#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {
    let telegram_bot = TelegramBot::try_new().await;

    let mut last_error = None;
    for try_run in 1..5 {
        if let Err(error) = handler::handle(&telegram_bot).await {
            eprintln!("Error: {} in try {}", error, try_run);
            last_error = Some(error);
        } else {
            break;
        }
        sleep(Duration::from_secs(10)).await;
    }

    if let Some(error) = last_error {
        telegram_bot
            .send_message(format!(
                "Failed to execute bot... see log for more information. {}",
                error
            ))
            .await;
    }
    
    Ok(())
}
