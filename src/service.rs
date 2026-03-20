use std::{process::exit, thread::sleep};

use log::{debug, error, info, warn};
use reqwest::Url;

use crate::{
    config::Config,
    diff::find_differences,
    json::{read_from_json, write_to_json},
    services::traits::{Entry, EntryId, ResponseData},
    telegram::{TELEGRAM_REQUEST_THROTTLE_SECONDS, send_message},
};

pub trait Service {
    type Entry: Entry;
    type ResponseData: ResponseData<Entry = Self::Entry>;

    fn update_known_entries(&self, config: Config) {
        info!("Starting {} update notifier bot", config.name);

        let read_result = Self::get_known_entries(&config);

        let known_entries = read_result.unwrap_or_else(|e| {
            warn!("Failed to get known entries: {:?}", e);
            info!("Initializing known entries storage");
            vec![]
        });

        info!("Total known entries: {}", known_entries.len());

        info!("Fetching entries from {}", config.name);

        let result = Self::fetch_entries(&config);

        let fetched_entries = match result {
            Ok(entries) => entries,
            Err(e) => {
                error!("Failed to fetch entries: {:?}", e);
                warn!("Exiting program");
                return;
            }
        };

        let filtered_new_entries = Self::filter_entries(&fetched_entries);

        filtered_new_entries.iter().for_each(|entry| {
            debug!("Fetched entry: {} ({})", entry.name(), entry.id());
        });

        if known_entries.is_empty() {
            let msg = format!(
                "{} is just initialized with {} entries",
                config.name,
                filtered_new_entries.len()
            );

            if let Err(e) = send_message(&msg) {
                error!("Failed to send Telegram message: {}", e);
            }

            write_to_json(&filtered_new_entries, config.cache_file_path).unwrap();

            return;
        }

        info!(
            "Total entries from {} API: {}",
            config.name,
            filtered_new_entries.len()
        );

        // Find differences between previous_data and response comparing by chain_id
        let (added_entries, removed_entries) =
            find_differences(&known_entries, &filtered_new_entries);

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
        match write_to_json(&filtered_new_entries, &config.cache_file_path) {
            Ok(_) => info!("Data saved to {}", config.cache_file_path),
            Err(e) => error!("Failed to save data to JSON: {}", e),
        }
    }

    fn get_known_entries(config: &Config) -> Result<Vec<Self::Entry>, Box<dyn std::error::Error>> {
        let raw_data: String = read_from_json(&config.cache_file_path)?;

        let known_entries: Vec<Self::Entry> = serde_json::from_str(&raw_data)?;

        Ok(known_entries)
    }

    fn find_entry_with_id(
        config: &Config,
        id: &EntryId,
    ) -> Result<Option<Self::Entry>, Box<dyn std::error::Error>> {
        let known_entries = Self::get_known_entries(config)?;

        let entry = known_entries
            .iter()
            .find(|entry| entry.id() == *id)
            .cloned();

        Ok(entry)
    }

    fn process_changes(
        &self,
        config: &Config,
        changed_entries: Vec<&Self::Entry>,
        msg_header: &str,
    ) {
        for changed_entry in changed_entries {
            sleep(TELEGRAM_REQUEST_THROTTLE_SECONDS);

            let result = Self::notify_on_change(&config, changed_entry, msg_header);

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

    fn fetch_entries(config: &Config) -> Result<Vec<Self::Entry>, reqwest::Error> {
        let url: Url;

        if let Some(data_url) = &config.data_url {
            url = Url::parse(data_url).expect(&format!(
                "Provided invalid URL for service {}",
                Self::name()
            ));
        } else {
            url = Self::get_default_data_endpoint_url()
        }

        let response = reqwest::blocking::get(url)?.error_for_status();

        match response {
            Ok(response) => {
                let data: Self::ResponseData = response.json()?;
                Ok(data.entries())
            }
            Err(e) => Err(e),
        }
    }

    fn notify_on_change(
        config: &Config,
        entry: &Self::Entry,
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

    fn filter_entries(entries: &[Self::Entry]) -> Vec<Self::Entry> {
        entries.to_vec()
    }

    fn get_default_data_endpoint_url() -> Url;

    fn name() -> &'static str;
}
