/*!
Structures for a registry of market identifiers.

This structure is based on the data provided by
[ISO 10383 - Market Identifier Code](https://www.iso20022.org/10383/iso-10383-market-identifier-codes)
and allows the lookup of valid and standard market codes.

The market registry is an implementation of the
[`Registry`](../registry/trait.Registry.html) trait for `Market` supporting the
lookup of MIC identifiers.
*/

use chrono::NaiveDate;

// ------------------------------------------------------------------------------------------------
// Public Types
// ------------------------------------------------------------------------------------------------

/// The current status of a market, the standard does not delete or
/// reuse identifiers, it only marks them as deleted or not operational.
#[derive(Clone)]
pub enum MarketStatus {
    Active,
    Deleted,
    NotOperational
}

/// Details of a market contained in the standard's Excel-based registry.
#[derive(Clone)]
pub struct Market {
    /// the MIC code, or identifier for the market
    pub mic: String,
    /// the ISO two-character country code
    pub country_code: String,
    /// human readable version of the country
    pub country: String,
    /// description of the market
    pub description: String,
    /// current status of the market
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
