/*!
A comprehensive data model for various financial data.

The purpose of this project is to construct a comprehensive data
model for company and market financial information in a coherent
and idiomatic manner, not of a specific service provider's API.

This model can then be populated using requests described with
traits and implemented by a given service provider. Thus, clients
can use the common data model with Rust-native types and idioms
but switch in different providers for different data types, markets,
or qualities of service.

This library only provides types and traits that can be implemented
by a `Provider` that executes requests for financial data such as
price quotes, analyst data, or company information. In the model
we use the term _request trait_ to indicate a trait that contains
functions that make a request for data and which use the common
`RequestResult` response.
*/

extern crate chrono;
#[macro_use]
extern crate lazy_static;
extern crate regex;
extern crate steel_cent;

use std::time::Duration;

use chrono::{DateTime, Local};

// ------------------------------------------------------------------------------------------------
// Public Types
// ------------------------------------------------------------------------------------------------

/// The time zone associated with all `chrono::DateTime` instances.
pub type ResponseTimezone = Local;

/// A snapshot value; `data` with a `date`, usually the last updated
/// or or calculated date and time.
#[derive(Debug)]
pub struct Snapshot<T> {
    pub date: DateTime<ResponseTimezone>,
    pub data: T
}

/// A time-bounded value; `data` with a `start_date` and `end_date`
/// signifying the range within which the data is considered valid.
#[derive(Debug)]
pub struct Bounded<T> {
    pub start_date: DateTime<ResponseTimezone>,
    pub end_date: DateTime<ResponseTimezone>,
    pub data: T
}

/// Represents a `series` of data points, over the time period indicated
/// by `interval`, with values separated by `intra_interval`.
#[derive(Debug)]
pub struct Series<I, T> {
    /// the interval over which time data is reported
    pub interval: I,
    /// the interval between data points within the overall `interval`
    pub intra_interval: Duration,
    /// the actual data points in increasing time order
    pub series: Vec<Snapshot<T>>
}

// ------------------------------------------------------------------------------------------------
// Public Modules
// ------------------------------------------------------------------------------------------------

pub mod symbol;

pub use symbol::{Symbol, Symbols};

pub mod analysis;

pub mod classification;

pub mod market;

pub mod quote;

pub mod registry;

pub mod reporting;

pub mod provider;

pub mod request;
