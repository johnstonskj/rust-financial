/*!
Provides the trait required by service provider implementations.


Before implementing any _request traits_ a service provider needs to
implement the `Provider` trait. This is the mechanism by which a client
instantiates a specific service provider.
*/

use crate::request::RequestResult;

// ------------------------------------------------------------------------------------------------
// Public Traits
// ------------------------------------------------------------------------------------------------

/// This trait allows a client to create a provider instance as
/// well as introspect basic descriptive properties.
pub trait Provider
    where Self: std::marker::Sized {

    /// Create a new instance, performing any initialization which _may_
    /// involve a network operation, to perform authentication for example.
    fn new() -> RequestResult<Self>;

    /// A string that can be used to name or describe the provider.
    fn attribution(&self) -> String;

    /// The URL to the provider's home page or documentation.
    fn url(&self) -> String;
}
