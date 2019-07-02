/*!
Provides structs and traits that represent common market analysis.
*/

use std::collections::HashMap;
use std::time::Duration;

use steel_cent::SmallMoney;
use chrono::DateTime;

use crate::{Bounded, ResponseTimezone, Snapshot, Symbol, Symbols};
use crate::request::RequestResult;
use crate::reporting::FinancialPeriod;

// ------------------------------------------------------------------------------------------------
// PUBLIC TYPES
// ------------------------------------------------------------------------------------------------

/// Used to count things.
pub type Counter = u32;

/// The type of analyst recommendation/position.
pub enum RatingType {
    Buy,
    Hold,
    Sell,
    Underweight,
    Overweight
}

/// The set of recommendation trends over some period of time.
pub struct Ratings {
    pub ratings: HashMap<RatingType, Counter>,
    pub scale_mark: Option<f32>,
    pub averaged_over: Option<Duration>
}

/// Consensus price targets; high, low, and average.
pub struct PriceTarget {
    pub high: SmallMoney,
    pub low:  SmallMoney,
    pub average: SmallMoney,
    pub number_of_analysts: Counter
}

/// Consensus Earnings per Share (EPS) targets for some fiscal period.
pub struct EPSConsensus {
    pub consensus: SmallMoney,
    pub number_of_estimates: Counter,
    pub fiscal_period: FinancialPeriod,
    pub fiscal_end_date: DateTime<ResponseTimezone>,
}

// ------------------------------------------------------------------------------------------------
// PUBLIC TRAITS
// ------------------------------------------------------------------------------------------------

/// This trait is implemented by providers to return a set of symbols that are expected
/// to represent peer companies to `for_symbol`. This set of peers could be provided by
/// the market or the service provider itself.
pub trait Peers {

    /// Return a set of peer symbols.
    fn peers(&self, for_symbol: Symbol) -> RequestResult<Snapshot<Symbols>>;
}

/// This trait is implemented by providers to return various analyst recommendations.
pub trait AnalystRecommendations {

    fn target_price(&self, for_symbol: Symbol) -> RequestResult<Snapshot<PriceTarget>>;

    fn consensus_rating(&self, for_symbol: Symbol) -> RequestResult<Bounded<Ratings>>;

    fn consensus_eps(&self, for_symbol: Symbol) -> RequestResult<EPSConsensus>;
}