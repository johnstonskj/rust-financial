/*!
Provides the common `Result` type and error enum used by _request traits_.
*/

#[derive(Debug)]
pub enum RequestError {
    ConfigurationError(String),
    CommunicationError,
    AuthenticationError,
    AuthorizationError,
    BadSymbolError(String),
    BadRequestError,
    BadResponseError,
    RequestThrottled
}

pub type RequestResult<T> = Result<T, RequestError>;

