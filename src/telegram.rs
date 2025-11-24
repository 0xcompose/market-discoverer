use std::time::Duration;

use log::{debug, error, info};
use reqwest;
use serde_json::json;

pub const TELEGRAM_REQUEST_THROTTLE_SECONDS: Duration = Duration::from_secs(1);

pub fn mock_send_message(message: &str) -> Result<(), Box<dyn std::error::Error>> {
    info!("Mocking message: {}", message);
    Ok(())
}

pub fn send_message(message: &str) -> Result<(), reqwest::Error> {
    dotenv::dotenv().ok();

    let bot_token = std::env::var("TG_BOT_TOKEN").expect("TG_BOT_TOKEN is not set");
    let chat_id = std::env::var("TG_CHAT_ID").expect("TG_CHAT_ID is not set");
    let thread_id = std::env::var("TG_THREAD_ID").expect("TG_THREAD_ID is not set");

    debug!("Chat ID: {}", chat_id);
    debug!("Thread ID: {}", thread_id);
    debug!("Message: {}", message);

    let url = format!("https://api.telegram.org/bot{}/sendMessage", bot_token);

    info!("Sending message to Telegram...");

    let client = reqwest::blocking::Client::new();

    let response = client
        .post(&url)
        .json(&json!({
            "chat_id": chat_id,
            "text": message,
            "message_thread_id": thread_id
        }))
        .send()?;

    info!("Message sent successfully");
    debug!("Response: {:?}", response);

    Ok(())
}
