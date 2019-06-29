use fin_model::provider::Provider;
use fin_model::request::RequestResult;

use crate::env;

// ------------------------------------------------------------------------------------------------

pub struct IEXProvider {
    pub host: String,
    pub version: String,
    pub token: String,
    pub default_currency: String
}

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
