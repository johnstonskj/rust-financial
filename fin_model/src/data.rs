use steel_cent::SmallMoney;
use chrono::{DateTime,Local};

use crate::{Symbol, Snapshot, Bounded};
use crate::quote::PriceBounds;
use crate::request::RequestResult;

pub struct Ratings {
    buy: u32,
    hold: u32,
    none: u32,
    sell: u32,
    overweight: u32,
    underweight: u32,
    scale_mark: f32
}

pub struct PriceTarget {
    pub target: PriceBounds,
    pub average: SmallMoney,
    number_of_analysts: u32
}

pub struct EPSConsensus {
    pub consensus: SmallMoney,
    number_of_estimates: u32,
    fiscal_period: String,
    fiscal_end_date: DateTime<Local>,
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