# Crate fin_model

This library only provides types and traits that can be implemented by a 
`Provider` that executes requests for financial data such as price quotes,
analyst data, or company information. In the model we use the term _request
trait_ to indicate a trait that contains functions that make a request for
data and which use the common `RequestResult` response. 

* `fin_model` core library and composable types.
* `::analysis` core analyst recommendations, `Ratings`, `PriceTarget`, 
  and `EPSConsensus`.
* `::classification` a type, `Code<T>`, and trait, `ClassificationScheme<T>`
  used to model classification schemes.
* `::market` a type, `Market`, and trait, `MarketRegistry` used to model
  registries for market/exchange information.
* `::provider` the core trait implemented by providers of the request traits
* `::quote` market quotes, `Quote`, `QuotePrice`, `PriceRange`, and 
  `PriceRangeSeries`.
* `::reporting` core types for reporting functions, `FinancialPeriod` and
  `FiscalPeriod`.
* `::request` result and error types for requests.