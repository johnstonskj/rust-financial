/*!
*/

use chrono::NaiveDate;

pub struct Exchange {
    pub mic: String,
    pub country_code: String,
    pub description: String,

    pub abbreviation: String,
    pub suffix: String,
    pub consolidated_tape_id: String
}

pub enum MarketRegistrationStatus {
    Active,
    Deleted
}

// [ISO 10383 Market Identifier Codes](https://www.iso20022.org/10383/iso-10383-market-identifier-codes)

pub struct MarketIdentifierCode {
    pub mic: String,
    pub country_code: String,
    pub country: String,
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

pub trait MICRegistry {

    fn new() -> Self;

    fn name() -> String;

    fn acronym() -> String;

    fn source() -> String;

    fn governing_body() -> Option<String>;

    fn last_updated() -> NaiveDate;

    fn next_publication() -> NaiveDate;

    fn get(&self, code: String) -> Option<MarketIdentifierCode>;
}

// ------------------------------------------------------------------------------------------------
// Optional Modules
// ------------------------------------------------------------------------------------------------

#[cfg(exchanges)]
pub mod exchanges;