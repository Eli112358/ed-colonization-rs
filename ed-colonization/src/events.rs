use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct JournalEntry {
    pub(crate) timestamp: String,
    pub(crate) event: String,
}
