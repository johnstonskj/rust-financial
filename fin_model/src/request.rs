/*!
Provides the common `Result` type and error enum used by _request traits_.

## Example

```rust
pub trait Peers {
    fn peers(&self, for_symbol: Symbol) -> RequestResult<Snapshot<Vec<Symbol>>>;
}
```
*/

// ------------------------------------------------------------------------------------------------
// PUBLIC TYPES
// ------------------------------------------------------------------------------------------------

/// The common error responses returned from _request traits_.
#[derive(Debug)]
pub enum RequestError {
    /// Usually a response from `Provider::new` this indicates that some
    /// error occurred due to missing or invalid configuration data.
    ConfigurationError(String),

    /// Failure to communicate with the service provider, where no additional
    /// information or specific type can be determined.
    CommunicationError,

    /// Failure to authenticate, this could be a wrapper around an OAuth
    /// error, or a missing/invalid token rejected by the service provider.
    AuthenticationError,

    /// Your current session/identity does not permit the requested operation.
    AuthorizationError,

    /// The symbol you passed to the `Provider` is not valid.
    BadSymbolError(String),

    /// Something in the request was incorrect. This error may be returned
    /// from the local `Provider` implementation or wrap an error returned
    /// from the service provider.
    BadRequestError,

    /// Something in the response from the service provider was incorrect.
    /// This error is _usually_ returned from the local `Provider`
    /// implementation due to de-serialization failure.
    BadResponseError,

    /// The service provider is throttling requests, either globally or
    /// specifically for your identity.
    RequestThrottled,

    /// This can be used to indicate that a given operation is unsupported
    /// by the `Provider`. This may mean that 1) it is not supported by the
    /// service provider itself, 2) it is not _yet_ implemented by the
    /// `Provider`, or 3) may never be implemented. A provider implementation
    /// that returns this error should indicate in documentation why
    /// this is the case.
    Unsupported
}

/// The common `Result` returned from _request traits_; the success type is
/// unspecified but the error is always `RequestError`.
pub type RequestResult<T> = Result<T, RequestError>;

