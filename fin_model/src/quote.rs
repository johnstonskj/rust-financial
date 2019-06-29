use steel_cent::SmallMoney;
use chrono::{DateTime,Local};

use crate::Symbol;
use crate::request::RequestResult;

pub struct Price {
    pub price: SmallMoney,
    pub time: DateTime<Local>,
    pub change: Option<SmallMoney>,
    pub percentage: Option<f32>
}

pub struct PriceBounds {
    pub high: SmallMoney,
    pub low: SmallMoney
}

pub struct QuotePrice {
    pub price: Price,
    pub bounds: PriceBounds
}

pub struct Quote {
    pub symbol: Symbol,
    pub price: QuotePrice,
    pub open: Price,
    pub close: Price,
    pub volume: u64,
    pub extended: Option<Price>
}

pub trait FetchQuote {
    fn latest_price_only(&self, for_symbol: Symbol) -> RequestResult<SmallMoney>;

    fn latest(&self, for_symbol: Symbol) -> RequestResult<Quote>;

    fn delayed(&self, for_symbol: Symbol) -> RequestResult<QuotePrice>;
}

pub trait FetchQuotes {
    fn intra_day(&self, for_symbol: Symbol) -> RequestResult<Vec<Quote>>;
}