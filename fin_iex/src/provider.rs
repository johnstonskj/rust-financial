/*!
An Implementation of the Financial Model `Provider` trait for IEX.

Note that this implementation uses a number of environment variables
for configuration of the IEX endpoint URL.

```bash
$ export IEX_HOST=cloud
$ export IEX_VERSION=stable
$ export RUST_LOG=info
$
$ IEX_TOKEN={your-dev-token} cargo run iext
```

*/

use std::collections::HashMap;

use steel_cent::currency::with_code;

use fin_model::provider::Provider;
use fin_model::request::{RequestError, RequestResult};

use crate::env;

// ------------------------------------------------------------------------------------------------
// Public Types & Traits
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
        debug!("IEXProvider::make_api_url path: {}, query_params: {:?}", path, query_params);
        let mut params: Vec<String> = match query_params {
            Some(qp) => qp.iter().map(|e| format!("{}={}", e.0, e.1)).collect(),
            None => Vec::new()
        };
        params.push(format!("token={}", self.token));
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
// Trait Implementations
// ------------------------------------------------------------------------------------------------

const ENV_HOST: &str = "IEX_HOST";
const ENV_VERSION: &str = "IEX_VERSION";
const ENV_TOKEN: &str = "IEX_TOKEN";

const DEFAULT_CURRENCY: &str = "USD";

impl Provider for IEXProvider {

    fn new() -> RequestResult<Self> {
        // https://iextrading.com/api-exhibit-a/
        let host = match env::get_from_environment(ENV_HOST, Some("cloud".to_string())) {
            Some(host) => {
                if host == "cloud" || host == "sandbox" {
                    debug!("setting IEX host to {}", host);
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
                    debug!("setting IEX version to {}", version);
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
                    debug!("setting IEX version to <<private>>");
                    token
                } else {
                    return Err(env::invalid_environment(ENV_TOKEN, token))
                }
            }
            None => return Err(env::missing_environment(ENV_TOKEN))
        };
        match with_code(DEFAULT_CURRENCY) {
            Some(_) => (),
            None => return Err(RequestError::ConfigurationError(format!("invalid currency code: {}", DEFAULT_CURRENCY)))
        }
        info!("IEXProvider::<Provider>::new host: {}, version: {}, token: {},default_currency: {}",
               host, version, token, DEFAULT_CURRENCY);
        Ok(IEXProvider { host, version, token, default_currency: DEFAULT_CURRENCY.to_string() })
    }

    fn attribution(&self) -> String {
        "Powered by IEX Cloud".to_string()
    }

    fn url(&self) -> String {
        "https://iexcloud.io/".to_string()
    }
}
