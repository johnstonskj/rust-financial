/*!
Model for a provider that implements the financial data traits.

```compile_fail
use some_provider::AProvider;

let provider = match AProvider::new() {
    Ok(provider) => provider,
    Err(err) => panic!("Error configuring provider, {:?}", err)
};

println!("{}. ({})", provider.attribution(), provider.url());
```
*/

use crate::request::RequestResult;

/// This trait allows a client to create a provider instance as
/// well as introspect basic descriptive properties.
pub trait Provider
    where Self: std::marker::Sized {

    /// Create a new instance, performing any initialization which _may_
    /// involve a network operation, to perform authentication for example.
    fn new() -> RequestResult<Self>;

    /// Return a string that can be used to describe the provider.
    fn attribution(&self) -> String;

    /// The URL to the provider's home page or documentation.
    fn url(&self) -> String;
}
