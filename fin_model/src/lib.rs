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

extern crate steel_cent;
extern crate chrono;

use chrono::{DateTime,Local};

// ------------------------------------------------------------------------------------------------
// Public Types
// ------------------------------------------------------------------------------------------------

/// A snapshot value; `data` with a `date`, usually the last updated
/// or or calculated date and time.
pub struct Snapshot<T> {
    pub date: DateTime<Local>,
    pub data: T
}

/// A time-bounded value; `data` with a `start_date` and `end_date`
/// signifying the range within which the data is considered valid.
pub struct Bounded<T> {
    pub start_date: DateTime<Local>,
    pub end_date: DateTime<Local>,
    pub data: T
}

// ------------------------------------------------------------------------------------------------
// Public Modules
// ------------------------------------------------------------------------------------------------

pub mod symbol;

pub use symbol::Symbol;

pub mod quote;

pub mod analysis;

pub mod provider;

pub mod request;
