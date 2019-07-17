/*!
Provides structs and traits that represent common market analysis.
*/

use std::collections::HashMap;

use crate::prelude::*;
use crate::reporting::FinancialPeriod;

// ------------------------------------------------------------------------------------------------
// Public Types
// ------------------------------------------------------------------------------------------------

/// Used to count things.
pub type Counter = u32;

/// The type of analyst recommendation/position.
#[derive(PartialEq, Eq, Hash)]
pub enum RatingType {
    Buy,
    Hold,
    Sell,
    Underweight,
    Overweight
}

/// The set of recommendation trends over some period of time.
pub struct Ratings {
    /// a mapping of available rating types to counts, not all types may be available
    pub ratings: HashMap<RatingType, Counter>,
    /// a standardized represention of the consensus of recommendations
    pub scale_mark: Option<f32>,
}

/// Consensus price targets; high, low, and average.
pub struct PriceTarget {
    pub high: Money,
    pub low:  Money,
    pub average: Money,
    pub number_of_analysts: Counter
}

/// Consensus Earnings per Share (EPS) targets for some fiscal period.
pub struct EPSConsensus {
    pub consensus: Money,
    pub number_of_estimates: Counter,
    pub fiscal_period: FinancialPeriod,
    pub fiscal_end_date: Date,
    pub next_report_date: Date
}

// ------------------------------------------------------------------------------------------------
// Public Traits
// ------------------------------------------------------------------------------------------------

/// This trait is implemented by providers to return a set of symbols that are expected
/// to represent peer companies to `for_symbol`. This set of peers could be provided by
/// the market or the service provider itself.
pub trait Peers {

    /// Return a set of peer symbols.
    fn peers(&self, for_symbol: Symbol) -> RequestResult<Symbols>;
}

/// This trait is implemented by providers to return various analyst recommendations.
pub trait AnalystRecommendations {

    fn target_price(&self, for_symbol: Symbol) -> RequestResult<Snapshot<PriceTarget>>;

    fn consensus_rating(&self, for_symbol: Symbol) -> RequestResult<Vec<Bounded<Ratings>>>;

    fn consensus_eps(&self, for_symbol: Symbol) -> RequestResult<Vec<EPSConsensus>>;
}