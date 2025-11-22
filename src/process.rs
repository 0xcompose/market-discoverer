use crate::config::Config;
use crate::diff::find_differences;
use crate::notification::notify_on_change;
use log::{debug, error, info, warn};
use reqwest::blocking::Response;
use std::error::Error;
use std::process::exit;
use std::thread::sleep;
use std::time::Duration;

use crate::clients::common::{Entry, ResponseData};
use crate::json::{read_from_json, write_to_json};
use crate::telegram::send_message;

const TELEGRAM_REQUEST_THROTTLE_SECONDS: Duration = Duration::from_secs(1);

pub fn update_known_entries<T: ResponseData>(config: Config) {
    info!("Starting {} update notifier bot", config.name);

    let read_result = read_previous_data::<T::Entry>(&config);

    let previous_data = match read_result {
        Ok(data) => data,
        Err(e) => {
            warn!("Failed to read previous data: {}", e);
            info!("Initializing storage");
            vec![]
        }
    };

    info!("Total known entries: {}", previous_data.len());

    info!("Fetching entries from {}", config.name);

    let result = fetch_entries::<T>(&config);

    let fetched_entries = match result {
        Ok(entries) => entries,
        Err(e) => {
            error!("Failed to fetch entries: {}", e);
            warn!("Exiting program");
            return;
        }
    };

    fetched_entries.iter().for_each(|entry| {
        debug!("Fetched entry: {} ({})", entry.name(), entry.id());
    });

    if previous_data.is_empty() {
        let msg = format!(
            "{} is just initialized with {} entries",
            config.name,
            fetched_entries.len()
        );

        if let Err(e) = send_message(&msg) {
            error!("Failed to send Telegram message: {}", e);
        }

        write_to_json(&fetched_entries, config.cache_file_path).unwrap();

        return;
    }

    info!(
        "Total entries from {} API: {}",
        config.name,
        fetched_entries.len()
    );

    // Find differences between previous_data and response comparing by chain_id
    let (added_entries, removed_entries) = find_differences(&previous_data, &fetched_entries);

    if added_entries.is_empty() && removed_entries.is_empty() {
        info!("No changes in entries of {} detected", config.name);
        if let Err(e) = send_message(&format!(
            "No changes in entries of {} detected",
            config.name
        )) {
            error!("Failed to send Telegram message: {}", e);
        }
    }

    info!(
        "Detected {} added and {} removed entries",
        added_entries.len(),
        removed_entries.len()
    );

    process_changes(&config, added_entries, "✅ *Entry Added*");
    process_changes(&config, removed_entries, "❌ *Entry Removed*");

    // Save to JSON file
    match write_to_json(&fetched_entries, &config.cache_file_path) {
        Ok(_) => info!("Data saved to {}", config.cache_file_path),
        Err(e) => error!("Failed to save data to JSON: {}", e),
    }
}

pub fn read_previous_data<E: Entry>(config: &Config) -> Result<Vec<E>, Box<dyn Error>> {
    let raw_data: String = read_from_json(&config.cache_file_path)?;

    let previous_data: Vec<E> = serde_json::from_str(&raw_data)?;

    Ok(previous_data)
}

pub fn fetch_entries<T: ResponseData>(config: &Config) -> Result<Vec<T::Entry>, reqwest::Error> {
    let response: Response = reqwest::blocking::get(&config.data_endpoint_url)?;

    let successful_response = response.error_for_status()?;

    let data: T = successful_response.json::<T>()?;

    Ok(data.entries())
}

pub fn process_changes<E: Entry>(config: &Config, changed_entries: Vec<&E>, msg_header: &str) {
    for changed_entry in changed_entries {
        sleep(TELEGRAM_REQUEST_THROTTLE_SECONDS);

        let result = notify_on_change::<E>(&config, changed_entry, msg_header);

        if let Err(e) = result {
            error!(
                "Failed to send Telegram notification on entry id {} with error: {}",
                changed_entry.id(),
                e
            );
            // Stop program execution on any error with notification for it to recover later
            exit(1);
        }
    }
}
