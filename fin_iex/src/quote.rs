/*!
IEX API wrapper
*/

use std::time::Duration;

use serde;
use serde::{Deserialize, Serialize};

use fin_model::prelude::*;
use fin_model::quote::*;
use fin_model::reporting::FinancialPeriod;
use fin_model::symbol::is_valid;

use crate::internal::convert::*;
use crate::internal::metric::{record_api_usage, record_api_use, ApiName};
use crate::internal::request;
use crate::IEXProvider;

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
    total_volume: Option<f64>,
}

#[serde(rename_all = "camelCase")]
#[derive(Serialize, Deserialize, Debug)]
pub struct IEXHistoricalPrice {
    pub date: String,
    pub label: String,

    pub open: f64,
    pub high: f64,
    pub low: f64,
    pub close: f64,
    pub volume: f64,

    #[serde(rename = "uOpen")]
    pub unadjusted_open: f64,
    #[serde(rename = "uHigh")]
    pub unadjusted_high: f64,
    #[serde(rename = "uLow")]
    pub unadjusted_low: f64,
    #[serde(rename = "uClose")]
    pub unadjusted_close: f64,
    #[serde(rename = "uVolume")]
    pub unadjusted_volume: f64,

    pub change: f64,
    pub change_percent: f64,
    pub change_over_time: f64,
}

type IEXHistoricalPrices = Vec<IEXHistoricalPrice>;

#[serde(rename_all = "camelCase")]
#[derive(Serialize, Deserialize, Debug)]
pub struct IEXIntradayPrice {
    pub date: String,
    pub minute: String,
    pub label: String,

    pub open: f64,
    pub high: f64,
    pub low: f64,
    pub close: f64,
    pub volume: f64,
    pub notional: f64,
    pub number_of_trades: f64,

    pub market_open: f64,
    pub market_high: f64,
    pub market_low: f64,
    pub market_close: f64,
    pub market_volume: f64,
    pub market_average: f64,
    pub market_notional: f64,
    pub market_number_of_trades: f64,
    pub market_change_over_time: f64,

    pub change: f64,
    pub change_percent: f64,
    pub change_over_time: f64,
}

type IEXIntradayPrices = Vec<IEXIntradayPrice>;

// ------------------------------------------------------------------------------------------------
// Trait Implementations
// ------------------------------------------------------------------------------------------------

impl FetchPriceQuote for IEXProvider {
    fn latest_price_only(&self, for_symbol: Symbol) -> RequestResult<Money> {
        debug!(
            "IEXProvider::<FetchPriceQuote>::latest_price_only for_symbol: {}",
            for_symbol
        );
        assert_is_valid!(for_symbol);

        let api_url = self.make_api_url(format!("stock/{}/price", for_symbol), None);

        match request::make_api_call(api_url) {
            Ok(raw_price) => match price_from_string(self.get_default_currency(), &raw_price) {
                Ok(price) => {
                    record_api_use(ApiName::Price);
                    Ok(price)
                }
                Err(err) => {
                    warn!("IEXProvider::<FetchPriceQuote>::latest_price_only parser error: {:?} in {}", err, raw_price);
                    Err(RequestError::BadResponseError)
                }
            },
            Err(err) => {
                warn!(
                    "IEXProvider::<FetchPriceQuote>::latest_price_only returned error: {:?}",
                    err
                );
                Err(err)
            }
        }
    }

    fn real_time(&self, for_symbol: Symbol) -> RequestResult<Quote> {
        debug!(
            "IEXProvider::<FetchPriceQuote>::real_time for_symbol: {}",
            for_symbol
        );
        assert_is_valid!(for_symbol);

        let api_url = self.make_api_url(format!("stock/{}/quote", for_symbol), None);

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
                            percentage: Some(quote.change_percent),
                        },
                        latest_source: source_from_string(&quote.latest_source),
                        trade_size: None,
                        previous_close_date: None,
                        extended: Some(QuotePrice {
                            price: price_from_float(dc, quote.extended_price)?,
                            change: None,
                            percentage: None,
                        }),
                    },
                })
            }
            Err(err) => {
                warn!(
                    "IEXProvider::<FetchPriceQuote>::real_time returned error: {:?}",
                    err
                );
                Err(err)
            }
        }
    }

    fn delayed(&self, for_symbol: Symbol) -> RequestResult<DelayedQuote> {
        debug!(
            "IEXProvider::<FetchPriceQuote>::delayed for_symbol: {}",
            for_symbol
        );
        assert_is_valid!(for_symbol);

        let api_url = self.make_api_url(format!("stock/{}/delayed-quote", for_symbol), None);

        let response: RequestResult<IEXDelayedQuote> = request::make_json_call(api_url);
        let dc = self.get_default_currency();
        match response {
            Ok(quote) => {
                record_api_use(ApiName::DelayedQuote);
                Ok(DelayedQuote {
                    date: date_from_timestamp(quote.delayed_price_time)?,
                    data: QuotePriceDelayed {
                        latest: QuotePrice {
                            price: price_from_float(dc, quote.delayed_price)?,
                            change: None,
                            percentage: None,
                        },
                        delayed_by: 15,
                        high: price_from_float(dc, quote.high)?,
                        low: price_from_float(dc, quote.low)?,
                        trade_size: Some(quote.delayed_size as u64),
                        volume: match quote.total_volume {
                            None => None,
                            Some(v) => Some(v as u64),
                        },
                        previous_close_date: None,
                    },
                })
            }
            Err(err) => {
                warn!(
                    "IEXProvider::<FetchPriceQuote>::delayed returned error: {:?}",
                    err
                );
                Err(err)
            }
        }
    }
}

impl FetchPriceRangeSeries for IEXProvider {
    fn intra_day(
        &self,
        for_symbol: Symbol,
        interval_minutes: u8,
    ) -> RequestResult<Option<PriceRangeSeries>> {
        debug!(
            "IEXProvider::<FetchPriceRangeSeries>::intra_day for_symbol: {}, interval: {}",
            for_symbol, interval_minutes
        );
        assert_is_valid!(for_symbol);

        let api_url = self.make_api_url(
            format!(
                "/stock/{}/intraday-prices?chartInterval={}",
                for_symbol, interval_minutes
            ),
            None,
        );

        let response: RequestResult<IEXIntradayPrices> = request::make_json_call(api_url);
        let dc = self.get_default_currency();
        match response {
            Ok(values) => {
                record_api_usage(ApiName::Intraday, values.len() as u16);
                let series: RequestResult<Vec<Snapshot<PriceRange>>> = values
                    .iter()
                    .map(|v| intraday_to_price_range(dc, v))
                    .collect();
                match series {
                    Ok(data) => Ok(Some(PriceRangeSeries {
                        interval: SeriesInterval::Day,
                        intra_interval: Some(Duration::new((60 * interval_minutes) as u64, 0)),
                        series: data,
                    })),
                    Err(err) => Err(err),
                }
            }
            Err(err) => {
                warn!(
                    "IEXProvider::<FetchPriceRangeSeries>::intra_day returned error: {:?}",
                    err
                );
                Err(err)
            }
        }
    }

    fn last(
        &self,
        for_symbol: Symbol,
        interval: SeriesInterval,
    ) -> RequestResult<PriceRangeSeries> {
        debug!(
            "IEXProvider::<FetchPriceRangeSeries>::last for_symbol: {}, interval: {:?}",
            for_symbol, interval
        );
        assert_is_valid!(for_symbol);

        let range = match interval {
            SeriesInterval::Day => "1d",
            SeriesInterval::FiveDays => "5d",
            SeriesInterval::OneMonth => "1m",
            SeriesInterval::ThreeMonths => "3m",
            SeriesInterval::SixMonths => "6m",
            SeriesInterval::YearToDate => "ytd",
            SeriesInterval::OneYear => "1y",
            SeriesInterval::TwoYears => "2y",
            SeriesInterval::FiveYears => "5y",
        };

        let api_url = self.make_api_url(
            format!("/stock/{}/chart/{}?chartByDay=true", for_symbol, range),
            None,
        );

        let response: RequestResult<IEXHistoricalPrices> = request::make_json_call(api_url);
        let dc = self.get_default_currency();
        match response {
            Ok(values) => {
                record_api_usage(ApiName::Historical, values.len() as u16);
                let series: RequestResult<Vec<Snapshot<PriceRange>>> = values
                    .iter()
                    .map(|v| historical_to_price_range(dc, v))
                    .collect();
                match series {
                    Ok(data) => Ok(PriceRangeSeries {
                        interval,
                        intra_interval: None,
                        series: data,
                    }),
                    Err(err) => Err(err),
                }
            }
            Err(err) => {
                println!(
                    "IEXProvider::<FetchPriceRangeSeries>::intra_day returned error: {:?}",
                    err
                );
                Err(err)
            }
        }
    }

    fn from(
        &self,
        for_symbol: Symbol,
        start_date: DateTime,
        interval: SeriesInterval,
    ) -> RequestResult<PriceRangeSeries> {
        debug!(
            "IEXProvider::<FetchPriceRangeSeries>::from for_symbol: {}, start: {}, interval: {:?}",
            for_symbol, start_date, interval
        );
        assert_is_valid!(for_symbol);
        Err(RequestError::Unsupported)
    }

    fn for_period(
        &self,
        for_symbol: Symbol,
        period: FinancialPeriod,
    ) -> RequestResult<PriceRangeSeries> {
        debug!(
            "IEXProvider::<FetchPriceRangeSeries>::for_period for_symbol: {}, period: {}",
            for_symbol, period
        );
        assert_is_valid!(for_symbol);
        Err(RequestError::Unsupported)
    }
}

// ------------------------------------------------------------------------------------------------
// Private Implementations
// ------------------------------------------------------------------------------------------------

fn intraday_to_price_range(
    dc: &String,
    v: &IEXIntradayPrice,
) -> RequestResult<Snapshot<PriceRange>> {
    Ok(Snapshot {
        date: datetime_from_string(&v.date, &format!("{}:00", v.minute))?,
        data: PriceRange {
            open: price_from_float(dc, v.high)?,
            close: price_from_float(dc, v.high)?,
            high: price_from_float(dc, v.high)?,
            low: price_from_float(dc, v.low)?,
            volume: Some(v.volume as u64),
        },
    })
}

fn historical_to_price_range(
    dc: &String,
    v: &IEXHistoricalPrice,
) -> RequestResult<Snapshot<PriceRange>> {
    Ok(Snapshot {
        date: datetime_from_date_string(&v.date)?,
        data: PriceRange {
            open: price_from_float(dc, v.high)?,
            close: price_from_float(dc, v.high)?,
            high: price_from_float(dc, v.high)?,
            low: price_from_float(dc, v.low)?,
            volume: Some(v.volume as u64),
        },
    })
}
