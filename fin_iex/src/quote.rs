/*!
IEX API wrapper
*/

use serde;
use serde::{Serialize, Deserialize};

use fin_model::prelude::*;
use fin_model::quote::*;
use fin_model::reporting::FinancialPeriod;
use fin_model::symbol::is_valid;

use crate::IEXProvider;
use crate::internal::convert::*;
use crate::internal::metric::{ApiName, record_api_use};
use crate::internal::request;

// ------------------------------------------------------------------------------------------------
// API Types (internal)
// ------------------------------------------------------------------------------------------------

#[serde(rename_all = "camelCase")]
#[derive(Serialize, Deserialize, Debug)]
struct IEXQuote {
    pub symbol: String,
    pub company_name: String,
    pub calculation_price: String,

    pub open: f64,
    pub open_time: f64,
    pub close: f64,
    pub close_time: f64,
    pub high: f64,
    pub low: f64,
    pub latest_price: f64,
    pub latest_source: String,
    pub latest_time: String,
    pub latest_update: f64,
    pub latest_volume: f64,
    pub change: f64,
    pub change_percent: f64,

    pub last_trade_time: f64,

    pub delayed_price: f64,
    pub delayed_price_time: f64,

    pub previous_close: f64,

    pub extended_price: f64,
    pub extended_price_time: f64,
    pub extended_change: f64,
    pub extended_change_percent: f64,

    pub iex_last_updated: f64,
    pub iex_realtime_price: f64,
    pub iex_realtime_size: f64,
    pub iex_market_percent: f64,
    pub iex_volume: f64,
    pub iex_bid_price: f64,
    pub iex_bid_size: f64,
    pub iex_ask_price: f64,
    pub iex_ask_size: f64,

    pub primary_exchange: Option<String>,
    pub sector: Option<String>,
    pub market_cap: f64,
    pub pe_ratio: f64,
    pub week52_high: f64,
    pub week52_low: f64,
    pub ytd_change: f64,
    pub avg_total_volume: f64,
}

#[serde(rename_all = "camelCase")]
#[derive(Serialize, Deserialize, Debug)]
struct IEXDelayedQuote {
    symbol: String,
    delayed_price: f64,
    high: f64,
    low: f64,
    delayed_size: f64,
    delayed_price_time: f64,
    processed_time: f64,
    total_volume: Option<f64>
}

// ------------------------------------------------------------------------------------------------
// Trait Implementations
// ------------------------------------------------------------------------------------------------

impl FetchPriceQuote for IEXProvider {

    fn latest_price_only(&self, for_symbol: Symbol) -> RequestResult<Money> {
        debug!("IEXProvider::<FetchPriceQuote>::latest_price_only for_symbol: {}", for_symbol);
        assert_is_valid!(for_symbol);

        let api_url = self.make_api_url(
            format!("stock/{}/price", for_symbol),
            None);

        match request::make_api_call(api_url) {
            Ok(raw_price) =>
                match price_from_string(self.get_default_currency(), &raw_price) {
                    Ok(price) => {
                        record_api_use(ApiName::Price);
                        Ok(price)
                    },
                    Err(err) => {
                        warn!("IEXProvider::<FetchPriceQuote>::latest_price_only parser error: {:?} in {}", err, raw_price);
                        Err(RequestError::BadResponseError)
                    }
                },
            Err(err) => {
                warn!("IEXProvider::<FetchPriceQuote>::latest_price_only returned error: {:?}", err);
                Err(err)
            }
        }
    }

    fn real_time(&self, for_symbol: Symbol) -> RequestResult<Quote> {
        debug!("IEXProvider::<FetchPriceQuote>::real_time for_symbol: {}", for_symbol);
        assert_is_valid!(for_symbol);

        let api_url = self.make_api_url(
            format!("stock/{}/quote", for_symbol),
            None);

        let response: RequestResult<IEXQuote> = request::make_json_call(api_url);
        let dc = self.get_default_currency();
        match response {
            Ok(quote) => {
                record_api_use(ApiName::Quote);
                Ok(Quote {
                    date: date_from_timestamp(quote.latest_update)?,
                    data: QuotePriceFull {
                        range: PriceRange {
                            open: price_from_float(dc, quote.high)?,
                            close: price_from_float(dc, quote.high)?,
                            high: price_from_float(dc, quote.high)?,
                            low: price_from_float(dc, quote.low)?,
                            volume: Some(quote.latest_volume as u64),
                        },
                        latest: QuotePrice {
                            price: price_from_float(dc, quote.latest_price)?,
                            change: Some(price_from_float(dc, quote.change)?),
                            percentage: Some(quote.change_percent)
                        },
                        latest_source: source_from_string(&quote.latest_source),
                        trade_size: None,
                        previous_close_date: None,
                        extended: Some(QuotePrice {
                            price: price_from_float(dc, quote.extended_price)?,
                            change: None,
                            percentage: None
                        })
                    }
                })
            },
            Err(err) => {
                warn!("IEXProvider::<FetchPriceQuote>::real_time returned error: {:?}", err);
                Err(err)
            }
        }
    }

    fn delayed(&self, for_symbol: Symbol) -> RequestResult<DelayedQuote> {
        debug!("IEXProvider::<FetchPriceQuote>::delayed for_symbol: {}", for_symbol);
        assert_is_valid!(for_symbol);

        let api_url = self.make_api_url(
            format!("stock/{}/delayed-quote", for_symbol),
            None);

        let response: RequestResult<IEXDelayedQuote> = request::make_json_call(api_url);
        match response {
            Ok(quote) => {
                record_api_use(ApiName::DelayedQuote);
                Ok(DelayedQuote {
                    date: date_from_timestamp(quote.delayed_price_time)?,
                    data: QuotePriceDelayed {
                        latest: QuotePrice {
                            price: price_from_float(self.get_default_currency(), quote.delayed_price)?,
                            change: None,
                            percentage: None
                        },
                        delayed_by: 15,
                        high: price_from_float(self.get_default_currency(), quote.high)?,
                        low: price_from_float(self.get_default_currency(), quote.low)?,
                        trade_size: Some(quote.delayed_size as u64),
                        volume: match quote.total_volume {
                            None => None,
                            Some(v) => Some(v as u64)
                        },
                        previous_close_date: None
                    }
                })
            },
            Err(err) => {
                warn!("IEXProvider::<FetchPriceQuote>::delayed returned error: {:?}", err);
                Err(err)
            }
        }
    }
}

impl FetchPriceRangeSeries for IEXProvider {

    fn intra_day(&self, for_symbol: Symbol) -> RequestResult<Option<PriceRangeSeries>> {
        debug!("IEXProvider::<FetchPriceRangeSeries>::intra_day for_symbol: {}",
               for_symbol);
        assert_is_valid!(for_symbol);
        Err(RequestError::Unsupported)
    }

    fn last(&self, for_symbol: Symbol, interval: SeriesInterval) -> RequestResult<PriceRangeSeries> {
        debug!("IEXProvider::<FetchPriceRangeSeries>::last for_symbol: {}, interval: {:?}",
               for_symbol, interval);
        assert_is_valid!(for_symbol);
        Err(RequestError::Unsupported)
    }

    fn from(&self, for_symbol: Symbol, start_date: DateTime, interval: SeriesInterval) -> RequestResult<PriceRangeSeries> {
        debug!("IEXProvider::<FetchPriceRangeSeries>::from for_symbol: {}, start: {}, interval: {:?}",
               for_symbol, start_date, interval);
        assert_is_valid!(for_symbol);
        Err(RequestError::Unsupported)
    }

    fn for_period(&self, for_symbol: Symbol, period: FinancialPeriod) -> RequestResult<PriceRangeSeries> {
        debug!("IEXProvider::<FetchPriceRangeSeries>::for_period for_symbol: {}, period: {}",
               for_symbol, period);
        assert_is_valid!(for_symbol);
        Err(RequestError::Unsupported)
    }
}