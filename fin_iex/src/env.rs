/*!
NOT PUBLIC
*/

use std::env;

use fin_model::request::RequestError;

pub fn missing_environment(env_key: &'static str) -> RequestError {
    RequestError::ConfigurationError(format!("no {} environment variable", env_key))
}

pub fn invalid_environment(env_key: &'static str, value: String) -> RequestError {
    RequestError::ConfigurationError(format!("invalid value {} for environment variable {}", value, env_key))
}

pub fn get_from_environment(env_key: &'static str, or_else: Option<String>) -> Option<String> {
    match env::var(env_key) {
        Err(_) => match or_else {
            Some(_) => or_else,
            _ => None
        },
        Ok(value) => Some(value)
    }
}

