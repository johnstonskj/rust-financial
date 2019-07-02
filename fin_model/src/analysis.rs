/*!
Provides structs and traits that represent common market analysis.
*/

use std::collections::HashMap;
use std::time::Duration;

use steel_cent::SmallMoney;
use chrono::DateTime;

use crate::{Bounded, ResponseTimezone, Snapshot, Symbol};
use crate::request::RequestResult;

// ------------------------------------------------------------------------------------------------
// PUBLIC TYPES
// ------------------------------------------------------------------------------------------------

/// Used to count things
pub type Counter = u32;

/// The type of analyst recommendation/position
pub enum RatingType {
    Buy,
    Hold,
    Sell,
    Underweight,
    Overweight
}

/// The set of recommendation trends over some period of time
pub struct Ratings {
    pub ratings: HashMap<RatingType, Counter>,
    pub scale_mark: Option<f32>,
    pub averaged_over: Option<Duration>
}

/// Consensus price targets; high, low, and average
pub struct PriceTarget {
    pub high: SmallMoney,
    pub low:  SmallMoney,
    pub average: SmallMoney,
    pub number_of_analysts: Counter
}

/// Consensus Earnings per Share (EPS) targets for some fiscal period
pub struct EPSConsensus {
    pub consensus: SmallMoney,
    pub number_of_estimates: Counter,
    pub fiscal_period: String,
    pub fiscal_end_date: DateTime<ResponseTimezone>,
}

// ------------------------------------------------------------------------------------------------
// PUBLIC TRAITS
// ------------------------------------------------------------------------------------------------

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