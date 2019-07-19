# Project rust-financial

[![travis.ci](https://travis-ci.org/johnstonskj/rust-financial.svg?branch=master)](https://travis-ci.org/johnstonskj/rust-financial)
[![Coverage Status](https://coveralls.io/repos/github/johnstonskj/rust-financial/badge.svg?branch=master)](https://coveralls.io/github/johnstonskj/rust-financial?branch=master)
![Minimum Rust Version](https://img.shields.io/badge/Min%20Rust-1.34-green.svg)
![mit License](https://img.shields.io/badge/license-mit-118811.svg)
[![GitHub stars](https://img.shields.io/github/stars/johnstonskj/rust-financial.svg)](https://github.com/johnstonskj/rust-financial/stargazers)

A comprehensive data model for various financial data, with a partial 
implementation using IEX as a service provider.

The purpose of this project is to construct a comprehensive data model, not of 
a specific API but the _right_ model we would like for financial data. This 
model can then be populated using requests described with traits and implemented 
by different service providers. Thus, clients can use the common data model with
Rust-native types and idioms but switch in different providers for different
data types, markets, or qualities of service.

## Crate fin_model

This library only provides types and traits that can be implemented by a 
`Provider` that executes requests for financial data such as price quotes,
analyst data, or company information. In the model we use the term _request
trait_ to indicate a trait that contains functions that make a request for
data and which use the common `RequestResult` response. 

See also [fin_model/README](fin_model/README.md).

## Crate fin_data

This is an implementation of the `ClassificationScheme` trait for the UK _Standard 
industrial classification of economic activities_ (UK SIC), the US _Standard 
Industrial Classification_ (US SIC) codes as well as the _North American Industry 
Classification System_ (NAICS). It also provides an implementation of the
`MarketRegistry` trait for _Market Identifier Code_ (MIC) data provided by ISO.

See also [fin_data/README](fin_data/README.md).

## Crate fin_iex

This is an implementation of a number of the _request traits_ in `fin_model`
calling the [IEX Cloud](https://iexcloud.io/). There is an existing IEX based
_proto_ crate, [iex-rs](https://github.com/samwho/iex-rs), I decided not to use it to 
keep the focus on the transformation from low-level to fin_model API.

See also [fin_iex/README](fin_iex/README.md).
