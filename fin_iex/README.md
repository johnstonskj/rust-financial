# Crate fin_iex

This is an implementation of a number of the _request traits_ in the 
[fin_model](https://crates.io/crates/fin_model) crate calling the 
[IEX Cloud](https://iexcloud.io/).

## Modules

* `::provider` provides an implementation of the `Provider` trait, `IEXProvider`

## Implemented Traits

* `fin_model::quote::FetchPriceQuote`
* `fin_model::quote::FetchPriceRangeSeries`
* `fin_model::analysis::Peers`
* `fin_model::analysis::AnalystRecommendations`

## Example

```rust
use fin_iex::IEXProvider;

use fin_model::provider::Provider;

let provider = match IEXProvider::new() {
    Ok(provider) => provider,
    Err(err) => panic!("Error configuring provider, {:?}", err)
};

println!("Provider = [{}]({}).", provider.attribution(), provider.url());
```
## Tools

The optional feature `iex-tool` builds a command-line tool that can 
exercise some of the APIs above.