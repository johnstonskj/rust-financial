# Crate fin_iex

This is an implementation of a number of the _request traits_ in `fin_model`
calling the [IEX Cloud](https://iexcloud.io/).

* `fin_iex` simply exports the `IEXProvider` type from `::provider`
* `::provider` provides an implementation of the `Provider` trait and a set of 
  request traits from `fin_model`
