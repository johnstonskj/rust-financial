/*!
Conversion functions from IEX to fin_model structures.
*/

use steel_cent::currency::Currency;
use steel_cent::currency::with_code;

use regex::Regex;

use fin_model::prelude::*;
use fin_model::quote::*;

// ------------------------------------------------------------------------------------------------
// Public Functions
// ------------------------------------------------------------------------------------------------

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

pub fn date_from_timestamp(ts: f64) -> RequestResult<DateTime> {
    Ok(DateTime::from_timestamp(ts.trunc() as i64, 0))
}

pub fn price_from_string(currency: &String, price: &String) -> RequestResult<Money> {
    lazy_static! {
        static ref RE: Regex = Regex::new(r"^(\d+)(\.(\d+))$").unwrap();
    }

    match RE.captures(price) {
        None => {
            warn!("doesn't look like a float: {}", price);
            Err(RequestError::BadResponseError)
        },
        Some(captures) => {
            let currency: Currency = with_code(currency).unwrap();
            if let Some(_) = captures.get(2) {
                Ok(Money::of_major_minor(
                    currency,
                    captures[1].parse::<i32>().unwrap(),
                    captures[3].parse::<i32>().unwrap()))
            } else {
                Ok(Money::of_major_minor(
                    currency,
                    captures[1].parse::<i32>().unwrap(),
                    0))
            }
        }
    }
}

pub fn price_from_float(currency: &String, price: f64) -> RequestResult<Money> {
    // this isn't efficient, but deconstructing floats is a black art
    price_from_string(currency, &format!("{}", price))
}