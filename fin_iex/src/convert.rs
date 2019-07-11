// ------------------------------------------------------------------------------------------------
// Private
// ------------------------------------------------------------------------------------------------

use chrono::prelude::*;

use steel_cent::SmallMoney;
use steel_cent::currency::with_code;

use regex::Regex;

use fin_model::quote::*;
use fin_model::request::{RequestError, RequestResult};

pub fn source_from_string(src: &String) -> QuoteSource {
    if src == "IEX real time price" {
        QuoteSource::RealTime
    } else if src == "15 minute delayed price" {
        QuoteSource::Delayed
    } else if src == "Close" {
        QuoteSource::Close
    } else if src == "Previous close" {
        QuoteSource::PreviousClose
    } else {
        QuoteSource::Unknown
    }
}

pub fn date_from_timestamp(ts: f64) -> RequestResult<DateTime<Local>> {
    Ok(Local.timestamp_millis(ts.trunc() as i64))
}

pub fn price_from_string(currency: &String, price: &String) -> RequestResult<SmallMoney> {
    lazy_static! {
        static ref RE: Regex = Regex::new(r"^(\d+)(\.(\d+))$").unwrap();
    }

    match RE.captures(price) {
        None => {
            warn!("doesn't look like a float: {}", price);
            Err(RequestError::BadResponseError)
        },
        Some(captures) =>
            if let Some(_) = captures.get(2) {
                Ok(SmallMoney::of_major_minor(
                    with_code(currency).unwrap(),
                    captures[1].parse::<i32>().unwrap(),
                    captures[3].parse::<i32>().unwrap()))
            } else {
                Ok(SmallMoney::of_major_minor(
                    with_code(currency).unwrap(),
                    captures[1].parse::<i32>().unwrap(),
                    0))
            }
    }
}

pub fn price_from_float(currency: &String, price: f64) -> RequestResult<SmallMoney> {
    // this isn't efficient, but deconstructing floats is a black art
    price_from_string(currency, &format!("{}", price))
}