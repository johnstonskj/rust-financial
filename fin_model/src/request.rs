#[derive(Debug)]
pub enum RequestError {
    ConfigurationError(String),
    CommunicationError,
    AuthenticationError,
    AuthorizationError,
    BadRequestError,
    BadResponseError,
    RequestThrottled
}

pub type RequestResult<T> = Result<T, RequestError>;

