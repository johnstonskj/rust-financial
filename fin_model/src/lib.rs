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

// ------------------------------------------------------------------------------------------------
// Public Modules
// ------------------------------------------------------------------------------------------------

pub mod prelude;

pub mod analysis;

pub mod classification;

pub mod company;

pub mod market;

pub mod quote;

pub mod registry;

pub mod reporting;

pub mod provider;

pub mod request;

pub mod symbol;

