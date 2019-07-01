/*!
An Implementation of the Financial Model `Provider` trait for IEX.
*/

use std::collections::HashMap;

use fin_model::provider::Provider;
use fin_model::request::RequestResult;

use crate::env;

// ------------------------------------------------------------------------------------------------
// PUBLIC TYPES
// ------------------------------------------------------------------------------------------------

/// This is the provider type for IEX, it contains communication parameters
/// and defaults used when making requests to the IEX endpoints.
pub struct IEXProvider {
    host: String,
    version: String,
    token: String,
    default_currency: String
}

impl IEXProvider {
    /// Construct a valid URL from the endpoint path and any additional query
    /// parameters, such as `format=json`.
    pub fn make_api_url(&self, path: String, query_params: Option<HashMap<String, String>>) -> String {
        let mut params: Vec<String> = match query_params {
            Some(qp) => qp.iter().map(|e| format!("{}={}", e.0, e.1)).collect(),
            None => Vec::new()
        };
        params.push(format!("token={}",self.token));
        format!(
            "https://{}.iexapis.com/{}/{}{}{}",
            self.host,
            self.version,
            path,
            if path.contains("?") { "&" } else { "?" },
            params.join("&"))
    }

    /// The default currency used to format currency values.
    pub fn get_default_currency(&self) -> &String {
        &self.default_currency
    }
}

// ------------------------------------------------------------------------------------------------

const ENV_HOST: &str = "IEX_HOST";
const ENV_VERSION: &str = "IEX_VERSION";
const ENV_TOKEN: &str = "IEX_TOKEN";

impl Provider for IEXProvider {

    fn new() -> RequestResult<Self> {
        // https://iextrading.com/api-exhibit-a/
        let host = match env::get_from_environment(ENV_HOST, Some("cloud".to_string())) {
            Some(host) => {
                if host == "cloud" || host == "sandbox" {
                    host
                } else {
                    return Err(env::invalid_environment(ENV_HOST, host))
                }
            },
            None => return Err(env::missing_environment(ENV_HOST))
        };
        let version = match env::get_from_environment(ENV_VERSION, Some("stable".to_string())) {
            Some(version) => {
                if version == "stable" || host == "beta" {
                    version
                } else {
                    return Err(env::invalid_environment(ENV_VERSION, version))
                }
            }
            None => return Err(env::missing_environment(ENV_VERSION))
        };
        let token = match env::get_from_environment(ENV_TOKEN, None) {
            Some(token) => {
                if token.starts_with("Tpk_") || token.starts_with("pk_") || token.starts_with("sk_") {
                    token
                } else {
                    return Err(env::invalid_environment(ENV_TOKEN, token))
                }
            }
            None => return Err(env::missing_environment(ENV_TOKEN))
        };
        Ok(IEXProvider { host, version, token, default_currency: "USD".to_string() })
    }

    fn attribution(&self) -> String {
        "Powered by IEX Cloud".to_string()
    }

    fn url(&self) -> String {
        "https://iexcloud.io/".to_string()
    }
}
