# rust-financial

A comprehensive data model for various financial data, with implementation 
using IEX.

The purpose of this project is to construct a comprehensive data model, not of 
a specific API but the _right_ model we would hope to see. This model can
then be populated using requests described with traits and implemented by
a given service provider. Thus, clients can use the common data model with
Rust-native types and idioms but switch in different providers for different
data types, markets, or qualities of service.

## Library fin_model

This library only provides types and traits that can be implemented by a 
`Provider` that executes requests for financial data such as price quotes,
analyst data, or company information. In the model we use the term _request
trait_ to indicate a trait that contains functions that make a request for
data and which use the common `RequestResult` response. 

* `fin_model` core library and composable types
* `::analysis` analyst data, `Ratings`, `PriceTarget`, and `EPSConsensus`
* `::quote` market quotes, `Quote`, and `QuotePrice`
* `::request` result types for requests
* `::provider` the core trait implemented by providers of the request traits

## Library fin_iex

This is an implementation of a number of the _request traits_ in `fin_model`
calling the [IEX Cloud](https://iexcloud.io/).

* `fin_iex` simply exports the `IEXProvider` type from `::provider`
* `::provider` provides an implementation of the `Provider` trait and a set of 
  request traits from `fin_model`

