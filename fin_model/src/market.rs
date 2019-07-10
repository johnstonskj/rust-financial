/*!
*/

use chrono::NaiveDate;

// ------------------------------------------------------------------------------------------------
// Public Types
// ------------------------------------------------------------------------------------------------

#[derive(Clone)]
pub enum MarketStatus {
    Active,
    Deleted,
    NotOperational
}

#[derive(Clone)]
pub struct Market {
    pub mic: String,
    pub country_code: String,
    pub country: String,
    pub description: String,
    pub status: Option<MarketStatus>,

    pub operating_mic: Option<String>,
    pub mic_type: Option<String>,
    pub acronym: Option<String>,
    pub city: Option<String>,
    pub website: Option<String>,
    pub last_updated: Option<NaiveDate>,
    pub created: Option<NaiveDate>,
    pub comments: Option<String>
}

// ------------------------------------------------------------------------------------------------
// Public Traits
// ------------------------------------------------------------------------------------------------

pub trait MarketRegistry {

    fn new() -> Self;

    fn name() -> String;

    fn acronym() -> String;

    fn source() -> String;

    fn governing_body() -> String;

    fn last_updated() -> NaiveDate;

    fn next_publication() -> NaiveDate;

    fn get(&self, code: String) -> Option<&Market>;
}
