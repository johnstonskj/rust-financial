use std::collections::HashMap;

use reqwest;

use fin_model::request::RequestResult;

use crate::IEXProvider;

pub fn make_api_url(provider: &IEXProvider, path: String, query_params: Option<HashMap<String, String>>) -> String {
    let mut params: Vec<String> = match query_params {
        Some(qp) => qp.iter().map(|e| format!("{}={}", e.0, e.1)).collect(),
        None => Vec::new()
    };
    params.push(format!("token={}",provider.token));
    format!(
        "https://{}.iexapis.com/{}/{}{}{}",
        provider.host,
        provider.version,
        path,
        if path.contains("?") { "&" } else { "?" },
        params.join("&"))
}

pub fn make_api_call(api: String) -> RequestResult<String> {
    info!("HTTP GET {}", api);
    Ok(reqwest::get(api.as_str()).unwrap().text().unwrap())
}
