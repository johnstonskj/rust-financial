# Crate fin_data

This is an implementation of the `ClassificationScheme` trait (from the 
[fin_model](https://crates.io/crates/fin_model) crate)
for the UK _Standard 
industrial classification of economic activities_ (UK SIC), the US _Standard 
Industrial Classification_ (US SIC) codes as well as the _North American Industry 
Classification System_ (NAICS). It also provides an implementation of the
`MarketRegistry` trait for _Market Identifier Code_ (MIC) data provided by ISO.

## Modules

* `::classifiers::uk_sic` the UK [Standard industrial classification of 
  economic activities](https://www.gov.uk/government/publications/standard-industrial-classification-of-economic-activities-sic)
  scheme.
* `::classifiers::us_sic` the US [Standard Industrial Classification 
  (SIC)](https://www.sec.gov/info/edgar/siccodes.htm) scheme.
* `::classifiers::naics` the [North American Industry Classification System 
  (NAICS)](https://www.census.gov/eos/www/naics) scheme.
* `::markets` an implementation of `MarketRegistry` to surface data from the 
  [ISO 10383 - Market Identifier Code](https://www.iso20022.org/10383/iso-10383-market-identifier-codes)
  standard for market codes.