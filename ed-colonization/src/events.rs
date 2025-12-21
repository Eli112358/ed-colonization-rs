use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct JournalEntry {
    pub(crate) timestamp: String,
    pub(crate) event: String,
}

#[derive(Debug, Deserialize)]
pub struct ConstructionResource {
    pub(crate) Name_Localised: String,
    pub(crate) RequiredAmount: u64,
    pub(crate) ProvidedAmount: u64,
    pub(crate) Payment: u64,
}

#[derive(Debug, Deserialize)]
pub struct ColonisationConstructionDepot {
    pub(crate) timestamp: String,
    pub(crate) MarketID: u64,
    pub(crate) ConstructionProgress: f64,
    pub(crate) ConstructionComplete: bool,
    pub(crate) ConstructionFailed: bool,
    pub(crate) ResourcesRequired: Vec<ConstructionResource>,
}
