use std::{process::exit, thread::sleep};

use log::{debug, error, info, warn};

use crate::{
    config::Config,
    diff::find_differences,
    json::{read_from_json, write_to_json},
    notification::notify_on_change,
    services::traits::{Entry, ResponseData},
    telegram::{TELEGRAM_REQUEST_THROTTLE_SECONDS, send_message},
};

pub trait Service {
    type Entry: Entry;
    type ResponseData: ResponseData<Entry = Self::Entry>;

    fn process(&self, config: Config) {
        self.update_known_entries(config);
    }

    fn update_known_entries(&self, config: Config) {
        info!("Starting {} update notifier bot", config.name);

        let read_result = self.read_previous_data(&config);

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

        let result = self.fetch_entries(&config);

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

        self.process_changes(&config, added_entries, "✅ *Entry Added*");
        self.process_changes(&config, removed_entries, "❌ *Entry Removed*");

        // Save to JSON file
        match write_to_json(&fetched_entries, &config.cache_file_path) {
            Ok(_) => info!("Data saved to {}", config.cache_file_path),
            Err(e) => error!("Failed to save data to JSON: {}", e),
        }
    }

    fn read_previous_data(
        &self,
        config: &Config,
    ) -> Result<Vec<Self::Entry>, Box<dyn std::error::Error>> {
        let raw_data: String = read_from_json(&config.cache_file_path)?;

        let previous_data: Vec<Self::Entry> = serde_json::from_str(&raw_data)?;

        Ok(previous_data)
    }

    fn process_changes(
        &self,
        config: &Config,
        changed_entries: Vec<&Self::Entry>,
        msg_header: &str,
    ) {
        for changed_entry in changed_entries {
            sleep(TELEGRAM_REQUEST_THROTTLE_SECONDS);

            let result = notify_on_change::<Self::Entry>(&config, changed_entry, msg_header);

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

    fn fetch_entries(&self, config: &Config) -> Result<Vec<Self::Entry>, reqwest::Error> {
        let response = reqwest::blocking::get(&config.data_endpoint_url)?.error_for_status();

        match response {
            Ok(response) => {
                let data: Self::ResponseData = response.json()?;
                Ok(data.entries())
            }
            Err(e) => Err(e),
        }
    }
}
