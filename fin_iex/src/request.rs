/*!
NOT PUBLIC
*/

use reqwest;

use fin_model::request::RequestResult;

// ------------------------------------------------------------------------------------------------

pub fn make_api_call(api: String) -> RequestResult<String> {
    info!("HTTP GET {}", api);
    Ok(reqwest::get(api.as_str()).unwrap().text().unwrap())
}
