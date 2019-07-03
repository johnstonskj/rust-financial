
// [ISO 10383 Market Identifier Codes](https://www.iso20022.org/10383/iso-10383-market-identifier-codes)
pub type MIC = String;

pub struct Exchange {
    pub mic: MIC,
    pub region: [char;2],
    pub description: String,

    pub abbreviation: String,
    pub suffix: String,
    pub consolidated_tape_id: String
}

pub enum MarketRegistrationStatus {
    Active,
    Deleted
}

pub struct MarketIdentifierRegistration {
    pub country_code: String,
    pub country: String,
    pub mic: String,
    pub description: String,
    pub status: String,

    pub operating_mic: Option<String>,
    pub mic_type: Option<String>,
    pub acronym: Option<String>,
    pub city: Option<String>,
    pub website: Option<String>,
    pub last_updated: Option<String>,
    pub created: Option<String>,
    pub comments: Option<String>
}

// ------------------------------------------------------------------------------------------------
// Optional Modules
// ------------------------------------------------------------------------------------------------

#[cfg(exchanges)]
pub mod exchanges;