/*!
Model for provider-neutral financial data


*/

extern crate steel_cent;
extern crate chrono;

use chrono::{DateTime,Local};

/// Type for a market ticker symbol
pub type Symbol = String;

/// Type for a qualified ticker symbol using the same format
/// for the market and symbol itself.
pub struct QualifiedSymbol {
    market: Symbol,
    symbol: Symbol
}

/// A snapshot value; data with a date, usually the last updated.
pub struct Snapshot<T> {
    date: DateTime<Local>,
    data: T
}

/// A time-bounded value; data with a start and end date
pub struct Bounded<T> {
    start_date: DateTime<Local>,
    end_date: DateTime<Local>,
    data: T
}

pub mod quote;

pub mod analysis;

pub mod provider;

pub mod request;
