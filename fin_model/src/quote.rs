/*!
Provides structs and traits that represent quotes for market prices.

The _request traits_, and associated types, declared here allow
for the fetching of price quotes for stocks and other traded
securities. This is a core capability for a number of financial
applications, stock trading, market trend analysis and more.

The traits include the following capabilities:

* Fetching a real time quote which contains the current
  price as well as open/close prices.
* Where delayed prices are the only choice, or perhaps are less
  expensive to fetch these are also available.
* Fetching a series of prices over time.

*/

use chrono::DateTime;

use steel_cent::SmallMoney;

use crate::{ResponseTimezone, Series, Snapshot, Symbol};
use crate::reporting::FinancialPeriod;
use crate::request::RequestResult;

// ------------------------------------------------------------------------------------------------
// Public Types
// ------------------------------------------------------------------------------------------------

/// A price range indicates a high and low value within the open and close
/// window provided.
///
/// The market window `open`/`close` are determined by whether the trading day
/// is still open, in which case the value of `close`
/// is the price at closing of the _previous trading day_, otherwise
/// it is the price at closing of the same trading day.
pub struct PriceRange {
    /// price at market opening
    pub open: SmallMoney,
    /// price at market closing
    pub close: SmallMoney,
    /// highest price within the market window
    pub high: SmallMoney,
    /// lowest price within the market window
    pub low: SmallMoney,
    /// the (optional) volume of trading within the market window
    pub volume: Option<u64>
}

/// A returned, real-time or delayed, price quote.
pub struct QuotePrice {
    /// the current price
    pub price: SmallMoney,
    /// the (optional) change in currency, since the last close
    pub change: Option<SmallMoney>,
    /// the (optional) change, in percentage, since the last close
    pub percentage: Option<f64>,
}

/// A returned, delayed full quote quote.
pub struct QuotePriceDelayed {
    /// the latest price
    pub latest: QuotePrice,
    /// number of minutes of delay (average)
    pub delayed_by: u16,
    /// highest price within the market window
    pub high: SmallMoney,
    /// lowest price within the market window
    pub low: SmallMoney,
    /// the (optional) number of trades at this latest price
    pub trade_size: Option<u64>,
    /// the (optional) volume of trading within the market window
    pub volume: Option<u64>,
    /// the (optional) previous close date
    pub previous_close_date: Option<DateTime<ResponseTimezone>>,
}

pub type DelayedQuote = Snapshot<QuotePriceDelayed>;

#[derive(Debug)]
pub enum QuoteSource {
    RealTime,
    Delayed,
    Close,
    PreviousClose,
    Unknown
}

pub struct QuotePriceFull {
    /// the range of prices for the trading day
    pub range: PriceRange,
    /// the latest price
    pub latest: QuotePrice,
    /// the source method for `latest`
    pub latest_source: QuoteSource,
    /// the (optional) number of trades at this latest price
    pub trade_size: Option<u64>,
    /// the (optional) previous close date
    pub previous_close_date: Option<DateTime<ResponseTimezone>>,
    /// the (optional) price during extended trading
    pub extended: Option<QuotePrice>
}

/// Represents a `QuotePrice` at a given point in time.
pub type Quote = Snapshot<QuotePriceFull>;

/// Common intervals for quote series data.
#[derive(Debug)]
pub enum SeriesInterval {
    Day,
    FiveDays,
    OneMonth,
    ThreeMonths,
    SixMonths,
    YearToDate,
    OneYear,
    TwoYears,
    FiveYears
}

/// A series of price ranges, used for both inter-day and intra-day data points.
pub type PriceRangeSeries = Series<SeriesInterval, PriceRange>;

// ------------------------------------------------------------------------------------------------
// Public Traits
// ------------------------------------------------------------------------------------------------

/// This trait is implemented by providers that are able to provide price quotes
/// either in real-time or delayed.
///
/// Most service providers charge extra for, or cap requests for, or even choose
/// not to provide real-time pricing and so the delayed option is usually a good
/// fall-back for most cases.
pub trait FetchPriceQuote {

    /// Return only the latest price, this may be delayed or real-time, simply
    /// the best available from the service provider.
    fn latest_price_only(&self, for_symbol: Symbol) -> RequestResult<SmallMoney>;

    /// Return a real-time price, or `RequestError::Unsupported` if the service
    /// provider cannot honor real-time requests.
    fn real_time(&self, for_symbol: Symbol) -> RequestResult<Quote>;

    /// Return a delayed price, or `RequestError::Unsupported` if the service
    /// provider does not provide delayed prices.
    fn delayed(&self, for_symbol: Symbol) -> RequestResult<DelayedQuote>;
}

/// This trait is implemented by providers that are able to provide price data
/// over time, either intra-day pricing or historical pricing.
///
/// Not all providers may have an option for intra-day requests.
pub trait FetchPriceRangeSeries {

    /// Return a series of intra-day prices for the current trading day, or
    /// `RequestError::Unsupported` if the service provider does not provide
    /// intra-day data.
    fn intra_day(&self, for_symbol: Symbol) -> RequestResult<Option<PriceRangeSeries>>;

    /// Return a series of prices for the specified `SeriesInterval` going back from
    /// the current day; for example _the last five days_ (`SeriesInterval::FiveDays`).
    fn last(&self, for_symbol: Symbol, interval: SeriesInterval) -> RequestResult<PriceRangeSeries>;

    /// Return a series of prices for the specified `SeriesInterval` starting from
    /// `start_date`. If the start_date is less than `interval` from the current
    /// date the series will be truncated.
    fn from(&self, for_symbol: Symbol, start_date: DateTime<ResponseTimezone>, interval: SeriesInterval) -> RequestResult<PriceRangeSeries>;

    /// Return a series of prices for the specified financial period. The series
    /// may be truncated if the period is not yet completed.
    fn for_period(&self, for_symbol: Symbol, period: FinancialPeriod) -> RequestResult<PriceRangeSeries>;
}