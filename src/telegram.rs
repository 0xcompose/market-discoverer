use log::{debug, error, info};
use reqwest;
use serde_json::json;

pub fn mock_send_message(message: &str) -> Result<(), Box<dyn std::error::Error>> {
    info!("Mocking message: {}", message);
    Ok(())
}

pub fn send_message(message: &str) -> Result<(), Box<dyn std::error::Error>> {
    dotenv::dotenv().ok();

    let bot_token = std::env::var("TG_BOT_TOKEN").map_err(|_| "TG_BOT_TOKEN is not set")?;
    let chat_id = std::env::var("TG_CHAT_ID").map_err(|_| "TG_CHAT_ID is not set")?;
    let thread_id = std::env::var("TG_THREAD_ID").map_err(|_| "TG_THREAD_ID is not set")?;

    debug!("Chat ID: {}", chat_id);
    debug!("Thread ID: {}", thread_id);
    debug!("Message: {}", message);

    let url = format!("https://api.telegram.org/bot{}/sendMessage", bot_token);

    info!("Sending message to Telegram...");

    let client = reqwest::blocking::Client::new();

    let request = client.post(&url).json(&json!({
        "chat_id": chat_id,
        "text": message,
        "message_thread_id": thread_id
    }));

    let response = match request.send() {
        Ok(response) => response,
        Err(e) => {
            error!("Failed to send message: {}", e);
            return Err(e.into());
        }
    };

    if response.status().is_success() {
        info!("Message sent successfully");
        debug!("Response: {:?}", response);
    } else {
        error!("Failed to send message. Status: {}", response.status());
    }

    Ok(())
}
