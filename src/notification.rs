use log::{error, info};

use crate::{config::Config, services::traits::Entry, telegram::send_message};

pub fn notify_on_change<T: Entry>(
    config: &Config,
    entry: &T,
    msg_header: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    info!(
        "Notifying on change for entry: {} ({})",
        entry.name(),
        entry.id()
    );

    let message = format!("⚙️ {}\n\n{}\n\n{}", config.name, msg_header, entry.format());

    if let Err(e) = send_message(&message) {
        error!("Failed to send Telegram notification: {}", e);
        return Err(e);
    }

    Ok(())
}
