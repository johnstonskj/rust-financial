/*!
NOT PUBLIC
*/

use reqwest;

use serde::de::DeserializeOwned;

use fin_model::request::{RequestError, RequestResult};

// ------------------------------------------------------------------------------------------------
// Public Functions
// ------------------------------------------------------------------------------------------------

pub fn make_api_call(api: String) -> RequestResult<String> {
    info!("reqwest::get {}", api);
    match reqwest::get(api.as_str()) {
        Err(err) => match err.status() {
            Some(s) => Err(RequestError::from_u16(s.as_u16()).unwrap()),
            None => Err(RequestError::CommunicationError)
        },
        Ok(mut r) => if r.status().is_success() {
            Ok(r.text().unwrap_or("".to_string()))
        } else {
            Err(RequestError::from_u16(r.status().as_u16()).unwrap())
        }
    }
}

pub fn make_json_call<T: DeserializeOwned>(api: String) -> RequestResult<T> {
    info!("reqwest::get {}", api);
    let client = reqwest::Client::new();
    match client.get(api.as_str()).send() {
        Err(err) => match err.status() {
            Some(s) => {
                warn!("response status {}", s);
                Err(RequestError::from_u16(s.as_u16()).unwrap())
            },
            None => Err(RequestError::CommunicationError)
        },
        Ok(mut r) => if r.status().is_success() {
            let json: reqwest::Result<T> = r.json();
            match json {
                Err(err) => {
                    warn!("response error: {:?}", err);
                    Err(RequestError::BadResponseError)
                },
                Ok(j) => Ok(j)
            }
        } else {
            Err(RequestError::from_u16(r.status().as_u16()).unwrap())
        }
    }
}