use log::info;

use crate::{config::Config, services::traits::Entry, telegram::send_message};

pub fn notify_on_change<T: Entry>(
    config: &Config,
    entry: &T,
    msg_header: &str,
) -> Result<(), reqwest::Error> {
    info!(
        "Notifying on change for entry: {} ({})",
        entry.name(),
        entry.id()
    );

    let message: String = format!("⚙️ {}\n\n{}\n\n{}", config.name, msg_header, entry.format());

    send_message(&message)?;

    Ok(())
}
