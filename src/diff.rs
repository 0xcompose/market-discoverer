use std::collections::HashMap;

use crate::clients::common::{Entry, EntryId};

pub fn find_differences<'a, E: Entry>(
    previous_data: &'a Vec<E>,
    fetched_entries: &'a Vec<E>,
) -> (Vec<&'a E>, Vec<&'a E>) {
    let previously_known_entries: HashMap<EntryId, &E> = previous_data
        .iter()
        .map(|entry| (entry.id(), entry))
        .collect();

    let new_entries: HashMap<EntryId, &E> = fetched_entries
        .iter()
        .map(|entry| (entry.id(), entry))
        .collect();

    let added_entries: Vec<&E> = new_entries
        .iter()
        .filter(|entry| !previously_known_entries.contains_key(entry.0))
        .map(|entry| entry.1.to_owned())
        .collect();

    let removed_entries: Vec<&E> = previously_known_entries
        .iter()
        .filter(|entry| !new_entries.contains_key(entry.0))
        .map(|entry| entry.1.to_owned())
        .collect();

    (added_entries, removed_entries)
}
