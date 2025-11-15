use log::{error, info};
use std::fs;
use std::path::Path;

pub fn read_from_json(path: &str) -> Result<String, std::io::Error> {
    info!("Reading from JSON file {}", path);

    let json = match fs::read_to_string(path) {
        Ok(json) => json,
        Err(e) => {
            error!("Error reading from JSON file: {}", e);
            return Err(e);
        }
    };

    Ok(json)
}

pub fn write_to_json<T: serde::Serialize, P: AsRef<Path>>(
    data: &T,
    path: P,
) -> std::io::Result<()> {
    info!("Writing to JSON file {}", path.as_ref().display());
    let json = serde_json::to_string_pretty(data)?;

    fs::write(path, json)
}
