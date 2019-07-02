/*!
NOT PUBLIC
*/

use steel_cent::SmallMoney;
use steel_cent::formatting::us_style; // TODO: i18n

use fin_model::Symbol;
use fin_model::quote::*;
use fin_model::request::{RequestError, RequestResult};
use fin_model::symbol::is_valid;

use crate::IEXProvider;
use crate::request;

// ------------------------------------------------------------------------------------------------

impl FetchPriceQuote for IEXProvider {

    fn latest_price_only(&self, for_symbol: Symbol) -> RequestResult<SmallMoney> {
        debug!("IEXProvider::<FetchPriceQuote>::latest_price_only for_symbol: {}", for_symbol);
        assert_is_valid!(for_symbol);

        let api_url = self.make_api_url(
            format!("stock/{}/price", for_symbol),
            None);

        match request::make_api_call(api_url) {
            Ok(raw_price) =>
                match us_style().parser().parse::<SmallMoney>(&format!("{}{}", self.get_default_currency(), raw_price)) {
                    Ok(price) => Ok(price),
                    Err(_) => Err(RequestError::BadResponseError)
                },
            Err(err) => Err(err)
        }
    }

    fn real_time(&self, for_symbol: Symbol) -> RequestResult<Quote> {
        debug!("IEXProvider::<FetchPriceQuote>::real_time for_symbol: {}", for_symbol);
        Err(RequestError::Unsupported)
    }

    fn delayed(&self, for_symbol: Symbol) -> RequestResult<Quote> {
        debug!("IEXProvider::<FetchPriceQuote>::delayed for_symbol: {}", for_symbol);
        Err(RequestError::Unsupported)
    }
}
