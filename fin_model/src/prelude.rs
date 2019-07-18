/*!
Re-exports base types required to use _request traits_.

This can be used before any of the request trait modules, it not only
exports all of the locally defined types but also a `Money` type from
the [steel_cent](https://docs.rs/steel-cent/0.2.3/steel_cent/) crate and
`Date` and `DateTime` types from the [chrono](https://docs.rs/crate/chrono/0.4.7)
crate.
*/

use std::time::Duration;

pub use steel_cent::SmallMoney as Money;

pub use chrono::NaiveDate as Date;
pub use chrono::NaiveDateTime as DateTime;

// ------------------------------------------------------------------------------------------------
// Public Types
// ------------------------------------------------------------------------------------------------

/// A snapshot value; `data` with a `date`, usually the last updated
/// or or calculated date and time.
#[derive(Debug)]
pub struct Snapshot<T> {
    pub date: DateTime,
    pub data: T,
}

/// A time-bounded value; `data` with a `start_date` and `end_date`
/// signifying the range within which the data is considered valid.
#[derive(Debug)]
pub struct Bounded<T> {
    pub start_date: DateTime,
    pub end_date: DateTime,
    pub data: T,
}

/// Represents a `series` of data points, over the time period indicated
/// by `interval`, with values separated by `intra_interval`.
#[derive(Debug)]
pub struct Series<I, T> {
    /// the interval over which time data is reported
    pub interval: I,
    /// the interval between data points within the overall `interval`
    pub intra_interval: Option<Duration>,
    /// the actual data points in increasing time order
    pub series: Vec<T>,
}

// ------------------------------------------------------------------------------------------------
// Re-Exported Types
// ------------------------------------------------------------------------------------------------

pub use crate::provider::Provider;

pub use crate::registry::Registry;

pub use crate::request::{RequestError, RequestResult};

pub use crate::symbol::{Symbol, Symbols};
