# Crate fin_model

The purpose of this API is to construct a comprehensive data
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

> Note: in the model we use the term _request trait_ to indicate 
> a trait that contains functions that make a request for
> data and which use the common `RequestResult` response.
 
## Modules

* `::analysis` core analyst recommendations, `Ratings`, `PriceTarget`, 
  and `EPSConsensus`.
* `::classification` a type, `Code<T>`, and trait, `ClassificationScheme<T>`
  used to model classification schemes.
* `::company` company information, income and balance sheets.
* `::market` a type, `Market`, and trait, `MarketRegistry` used to model
  registries for market/exchange information.
* `::provider` the core trait implemented by providers of the request traits
* `::quote` market quotes, `Quote`, `QuotePrice`, `PriceRange`, and 
  `PriceRangeSeries`.
* `::reporting` core types for reporting functions, `FinancialPeriod` and
  `FiscalPeriod`.
* `::request` result and error types for requests.
* `::symbol` types for market and security symbols.

A common subset of the types declared in the modules above can be
imported from the `::prelude` module. 

## Example

TBD