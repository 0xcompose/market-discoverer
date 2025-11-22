use serde::{Serialize, de::DeserializeOwned};
use std::fmt::Debug;

pub type EntryId = String;

pub trait Entry: Serialize + DeserializeOwned + Debug {
    fn id(&self) -> EntryId;

    fn name(&self) -> String;

    fn format(&self) -> String;
}

pub trait ResponseData: Serialize + DeserializeOwned + Debug {
    type Entry: Entry;

    fn entries(&self) -> Vec<Self::Entry>;
}
