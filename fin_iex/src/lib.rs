/*!
An implementation of a number of the _request traits_ in the
`fin_model` library calling the [IEX Cloud](https://iexcloud.io/).

## Example

```no_run
use fin_iex::IEXProvider;

use fin_model::provider::Provider;

let provider = match IEXProvider::new() {
    Ok(provider) => provider,
    Err(err) => panic!("Error configuring provider, {:?}", err)
};

println!("Provider = [{}]({}).", provider.attribution(), provider.url());
```
*/

#[macro_use]
extern crate log;

extern crate steel_cent;
extern crate chrono;
extern crate reqwest;
extern crate serde;
extern crate serde_json;

#[macro_use]
extern crate lazy_static;
extern crate regex;

#[macro_use]
extern crate fin_model;

// ------------------------------------------------------------------------------------------------
// Public Modules/Exports
// ------------------------------------------------------------------------------------------------

pub mod provider;

pub use provider::IEXProvider;

// ------------------------------------------------------------------------------------------------
// Trait Implementation Modules
// ------------------------------------------------------------------------------------------------

mod quote;

// ------------------------------------------------------------------------------------------------
// Private Modules
// ------------------------------------------------------------------------------------------------

mod env;

mod request;

