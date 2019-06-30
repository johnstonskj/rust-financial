/*!
Provides structs and traits that represent common market analysis.
*/

use steel_cent::SmallMoney;
use chrono::{DateTime,Local};

use crate::{Symbol, Snapshot, Bounded};
use crate::quote::PriceBounds;
use crate::request::RequestResult;

pub type Counter = u32;

pub struct Ratings {
    pub buy: Counter,
    pub hold: Counter,
    pub none: Counter,
    pub sell: Counter,
    pub overweight: Counter,
    pub underweight: Counter,
    pub scale_mark: f32
}

pub struct PriceTarget {
    pub target: PriceBounds,
    pub average: SmallMoney,
    pub number_of_analysts: Counter
}

pub struct EPSConsensus {
    pub consensus: SmallMoney,
    pub number_of_estimates: Counter,
    pub fiscal_period: String,
    pub fiscal_end_date: DateTime<Local>,
}

pub trait Peers {
    fn peers(&self, for_symbol: Symbol) -> RequestResult<Snapshot<Vec<Symbol>>>;
}

pub trait Target {
    fn target(&self, for_symbol: Symbol) -> RequestResult<Snapshot<PriceTarget>>;
}

pub trait Rating {
    fn rating(&self, for_symbol: Symbol) -> RequestResult<Bounded<Ratings>>;
}

pub trait Consensus {
    fn eps_consensus(&self, for_symbol: Symbol) -> RequestResult<EPSConsensus>;
}